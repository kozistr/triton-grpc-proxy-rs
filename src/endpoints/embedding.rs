use std::collections::HashMap;

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use ndarray::{Array, Array1};
use triton_client::inference::model_infer_request::{InferInputTensor, InferRequestedOutputTensor};
use triton_client::inference::{ModelInferRequest, ModelInferResponse};

lazy_static! {
    pub static ref CLIENT: AsyncOnce<triton_client::Client> = {
        AsyncOnce::new(async {
            triton_client::Client::new("http://127.0.0.1:8001", None)
                .await
                .unwrap()
        })
    };
}

async fn inference(queries: Vec<String>) -> ModelInferResponse {
    let inputs: Vec<InferInputTensor> = vec![InferInputTensor {
        name: "text".into(),
        shape: vec![queries.len() as i64, 1],
        parameters: HashMap::new(),
        datatype: "BYTES".into(),
        contents: None,
    }];

    let request: ModelInferRequest = ModelInferRequest {
        model_name: "model".into(),
        model_version: 1.to_string(),
        id: "".into(),
        parameters: HashMap::new(),
        inputs,
        outputs: vec![InferRequestedOutputTensor {
            name: "embedding".into(),
            parameters: HashMap::new(),
        }],
        raw_input_contents: queries
            .into_iter()
            .map(|query: String| query.as_bytes().to_owned())
            .collect(),
    };

    CLIENT.get().await.model_infer(request).await.unwrap()
}

pub async fn get_embedding(queries: Vec<String>) -> Vec<Array1<f32>> {
    let response: ModelInferResponse = inference(queries).await;

    response
        .raw_output_contents
        .iter()
        .map(|r: &Vec<u8>| {
            let e: &[f32] = bytemuck::cast_slice::<u8, f32>(r.as_slice());
            Array::from_vec(e.to_vec())
        })
        .collect::<Vec<Array1<f32>>>()
}
