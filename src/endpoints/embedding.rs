use std::collections::HashMap;

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use ndarray::{Array, Array1, Array2};
use triton_client::inference::model_infer_request::{InferInputTensor, InferRequestedOutputTensor};
use triton_client::inference::{ModelInferRequest, ModelInferResponse};

use crate::constants::{TRITON_SERVER_URL, V1_EMBEDDING_SIZE};

lazy_static! {
    pub static ref CLIENT: AsyncOnce<triton_client::Client> = {
        AsyncOnce::new(async {
            triton_client::Client::new(TRITON_SERVER_URL, None)
                .await
                .unwrap()
        })
    };
}

fn serialize_to_byte_string(queries: Vec<String>) -> Vec<u8> {
    let total_len: usize = queries.iter().map(|query: &String| 4 + query.len()).sum();
    let mut len_bytes: Vec<u8> = Vec::with_capacity(total_len);

    for query in queries {
        len_bytes.extend_from_slice(&(query.len() as u32).to_le_bytes());
        len_bytes.extend_from_slice(query.as_bytes());
    }

    len_bytes
}

async fn inference(queries: Vec<String>) -> ModelInferResponse {
    let request: ModelInferRequest = ModelInferRequest {
        model_name: "model".into(),
        model_version: 1.to_string(),
        id: "".into(),
        parameters: HashMap::new(),
        inputs: vec![InferInputTensor {
            name: "text".into(),
            shape: vec![queries.len() as i64, 1],
            parameters: HashMap::new(),
            datatype: "BYTES".into(),
            contents: None,
        }],
        outputs: vec![InferRequestedOutputTensor {
            name: "embedding".into(),
            parameters: HashMap::new(),
        }],
        raw_input_contents: vec![serialize_to_byte_string(queries)],
    };

    CLIENT
        .get()
        .await
        .model_infer(request)
        .await
        .expect("failed to inference")
}

pub async fn get_embedding(queries: Vec<String>) -> Array2<f32> {
    let batch_size: usize = queries.len();

    let response: ModelInferResponse = inference(queries).await;

    let flatten_vectors: Vec<Array1<f32>> = response
        .raw_output_contents
        .into_iter()
        .map(|r: Vec<u8>| {
            let e: &[f32] = bytemuck::cast_slice::<u8, f32>(r.as_slice());
            Array::from_vec(e.to_owned())
        })
        .collect::<Vec<Array1<f32>>>();

    flatten_vectors
        .first()
        .expect("empty vectors")
        .to_owned()
        .into_shape((batch_size, V1_EMBEDDING_SIZE))
        .expect("failed to reshape")
}
