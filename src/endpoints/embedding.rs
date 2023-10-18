use std::collections::HashMap;

use ntex::web::types::State;
use triton_client::inference::model_infer_request::{InferInputTensor, InferRequestedOutputTensor};
use triton_client::inference::{ModelInferRequest, ModelInferResponse};

use crate::configs::Config;
use crate::models::EmbeddingResponse;

fn serialize_to_byte_string(queries: Vec<String>) -> Vec<u8> {
    let total_len: usize = queries.iter().map(|query: &String| 4 + query.len()).sum();
    let mut len_bytes: Vec<u8> = Vec::with_capacity(total_len);

    for query in queries {
        len_bytes.extend_from_slice(&(query.len() as u32).to_le_bytes());
        len_bytes.extend_from_slice(query.as_bytes());
    }

    len_bytes
}

pub async fn get_embedding(
    queries: Vec<String>,
    client: State<triton_client::Client>,
    config: State<Config>,
) -> Vec<EmbeddingResponse> {
    let batch_size: usize = queries.len();
    let embedding_size: usize = config.embedding_size;

    let request: ModelInferRequest = ModelInferRequest {
        model_name: config.model_name.clone(),
        model_version: config.model_version.clone(),
        id: "".into(),
        parameters: HashMap::new(),
        inputs: vec![InferInputTensor {
            name: config.input_name.clone(),
            shape: vec![queries.len() as i64, 1],
            parameters: HashMap::new(),
            datatype: "BYTES".into(),
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

    let mut flatten_vectors: Vec<f32> = Vec::with_capacity(batch_size * embedding_size);

    for r in response.raw_output_contents.into_iter() {
        let e: &[f32] = bytemuck::cast_slice::<u8, f32>(r.as_slice());
        flatten_vectors.extend_from_slice(e);
    }

    flatten_vectors
        .chunks_exact(embedding_size)
        .map(|row: &[f32]| EmbeddingResponse { embedding: row.to_vec() })
        .collect()
}
