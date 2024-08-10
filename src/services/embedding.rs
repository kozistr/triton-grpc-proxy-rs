use std::collections::HashMap;
use std::time::Instant;

use ntex::web::types::{Json, State};
use ntex::web::{post, Error, HttpResponse, ServiceConfig};
use triton_client::inference::model_infer_request::{InferInputTensor, InferRequestedOutputTensor};
use triton_client::inference::{ModelInferRequest, ModelInferResponse};

use crate::configs::Config;
use crate::models::{EmbeddingResponse, QueryRequest};

fn serialize_to_byte_string(queries: &[&str]) -> Vec<u8> {
    let total_len: usize = queries.iter().map(|query: &&str| 4 + query.len()).sum();
    let mut payload: Vec<u8> = Vec::with_capacity(total_len);

    for query in queries {
        payload.extend_from_slice(&(query.len() as u32).to_le_bytes());
        payload.extend_from_slice(query.as_bytes());
    }

    payload
}

async fn get_embeddings_from_triton_server(
    queries: &[&str],
    client: &State<triton_client::Client>,
    config: &State<Config>,
) -> Vec<EmbeddingResponse> {
    let batch_size: usize = queries.len();
    let embedding_size: usize = config.embedding_size;

    metrics::histogram!("tgp_batch_size").record(batch_size as f64);

    let start_time = Instant::now();
    let request: ModelInferRequest = ModelInferRequest {
        model_name: config.model_name.clone(),
        model_version: config.model_version.clone(),
        id: String::new(),
        parameters: HashMap::new(),
        inputs: vec![InferInputTensor {
            name: config.input_name.clone(),
            shape: vec![batch_size as i64, 1],
            parameters: HashMap::new(),
            datatype: "BYTES".to_string(),
            contents: None,
        }],
        outputs: vec![InferRequestedOutputTensor {
            name: config.output_name.clone(),
            parameters: HashMap::new(),
        }],
        raw_input_contents: vec![serialize_to_byte_string(queries)],
    };
    metrics::histogram!("tgp_serialization_duration").record(start_time.elapsed().as_secs_f64());

    let start_time = Instant::now();
    let response: ModelInferResponse =
        client.model_infer(request).await.expect("failed to inference");
    metrics::histogram!("tgp_inference_duration").record(start_time.elapsed().as_secs_f64());

    let mut vectors: Vec<f32> = Vec::with_capacity(batch_size * embedding_size);

    for r in &response.raw_output_contents {
        let e: &[f32] = bytemuck::cast_slice::<u8, f32>(r);
        vectors.extend_from_slice(e);
    }

    vectors
        .chunks_exact(embedding_size)
        .map(|row: &[f32]| EmbeddingResponse { embedding: row.to_vec() })
        .collect()
}

#[utoipa::path(
    post,
    path = "/v1/embedding",
    request_body = Vec<QueryRequest>,
    responses(
      (status = 200, description = "Get embeddings", body = Vec<EmbeddingResponse>),
    ),
  )]
#[post("/v1/embedding")]
pub async fn get_embeddings(
    requests: Json<Vec<QueryRequest>>,
    client: State<triton_client::Client>,
    config: State<Config>,
) -> Result<HttpResponse, Error> {
    metrics::counter!("tgp_request_count", "method" => "batch").increment(1);

    let queries: Vec<&str> = requests.iter().map(|req: &QueryRequest| req.query.as_str()).collect();

    let responses: Vec<EmbeddingResponse> =
        get_embeddings_from_triton_server(&queries, &client, &config).await;

    Ok(HttpResponse::Ok().json(&responses))
}

pub fn ntex_config(cfg: &mut ServiceConfig) {
    cfg.service(get_embeddings);
}
