use std::collections::HashMap;
use std::error::Error;

use ndarray::{Array, Array1};
use tonic::Status;
use triton_client::inference::model_infer_request::{InferInputTensor, InferRequestedOutputTensor};
use triton_client::inference::{ModelInferRequest, ModelInferResponse};

type InferResponse = Result<ModelInferResponse, triton_client::client::Error>;

thread_local! {
    pub static TRITON_CLIENT: triton_client::Client = triton_client::Client::new("http://192.168.219.107:8001", None).await?;
}

async fn request(queries: Vec<String>) -> InferResponse {
    let inputs: Vec<InferInputTensor> = vec![InferInputTensor {
        name: "text".into(),
        shape: vec![queries.len() as i64, 1],
        parameters: HashMap::new(),
        datatype: "BYTES".into(),
        contents: None,
    }];

    let request: ModelInferRequest = ModelInferRequest {
        id: "".into(),
        model_name: "model".into(),
        model_version: 1.to_string(),
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

    match TRITON_CLIENT.model_infer(request).await {
        Ok(response) => Ok(response),
        Err(e) => Err(e),
    }
}

async fn parse_inference_response(
    response: InferResponse,
) -> Result<Vec<Array1<f32>>, Box<dyn Error + Send + Sync>> {
    match response {
        Ok(response) => {
            let vectors = response
                .raw_output_contents
                .iter()
                .map(|r: &Vec<u8>| {
                    let e: &[f32] = bytemuck::cast_slice::<u8, f32>(r.as_slice());
                    Array::from_vec(e.to_vec())
                })
                .collect::<Vec<Array1<f32>>>();
            Ok(vectors)
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn get_embedding(queries: Vec<String>) -> Vec<Array1<f32>> {
    let embedding_request = request(queries);

    let response: ModelInferResponse = match embedding_request.await {
        Ok(res) => res,
        Err(e) => return Err(Status::internal(e.to_string())),
    };

    parse_inference_response(Ok(response)).await.unwrap()
}
