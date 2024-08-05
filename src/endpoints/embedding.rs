use std::collections::HashMap;

use ntex::web::types::State;
use triton_client::inference::model_infer_request::{InferInputTensor, InferRequestedOutputTensor};
use triton_client::inference::{ModelInferRequest, ModelInferResponse};

use crate::configs::Config;
use crate::models::EmbeddingResponse;

fn serialize_to_byte_string(queries: &[&str]) -> Vec<u8> {
    let total_len: usize = queries.iter().map(|query: &&str| 4 + query.len()).sum();
    let mut payload: Vec<u8> = Vec::with_capacity(total_len);

    for query in queries {
        payload.extend_from_slice(&(query.len() as u32).to_le_bytes());
        payload.extend_from_slice(query.as_bytes());
    }

    payload
}

pub async fn get_embeddings_from_triton_server(
    queries: &[&str],
    client: &State<triton_client::Client>,
    config: &State<Config>,
) -> Vec<EmbeddingResponse> {
    let batch_size: usize = queries.len();
    let embedding_size: usize = config.embedding_size;

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

    let response: ModelInferResponse =
        client.model_infer(request).await.expect("failed to inference");

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
