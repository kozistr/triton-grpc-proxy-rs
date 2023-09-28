use std::collections::HashMap;
use std::error::Error;

use ndarray::{Array, Array1};
use triton_client::inference::model_infer_request::{InferInputTensor, InferRequestedOutputTensor};
use triton_client::inference::{ModelInferRequest, ModelInferResponse};

use tonic::Status;

type InferResponse = Result<ModelInferResponse, triton_client::client::Error>;

async fn request(
    client: &mut triton_client::Client,
    queries: Vec<String>,
) -> InferResponse {
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

    match client.model_infer(request).await {
        Ok(response) => Ok(response),
        Err(e) => Err(e),
    }
}

async fn parse_inference_response(response: InferResponse) -> Result<Vec<Array1<f32>>, Box<dyn Error + Send + Sync>> {
    match response {
        Ok(response) => {
            let vectors = response.raw_output_contents.iter().map(|r: &Vec<u8>| {
                let e: &[f32] = bytemuck::cast_slice::<u8, f32>(r.as_slice());
                Array::from_vec(e.to_vec())
            }).collect::<Vec<Array1<f32>>>();
            Ok(vectors)
        },
        Err(e) => Err(Box::new(e)),
    }
}

async fn init(url: &str) -> anyhow::Result<triton_client::Client> {
    let triton_client: triton_client::Client = triton_client::Client::new(url, None).await?;
    Ok(triton_client)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client: triton_client::Client = init("http://192.168.219.107:8001").await.unwrap();

    let embedding_request = request(
        &mut client,
        vec!["asdf asdf asdf".to_owned(), "asdf asdf".to_owned()],
    );

    let response: ModelInferResponse = match embedding_request.await {
        Ok(res) => res,
        Err(e) => return Err(Status::internal(e.to_string())),
    };

    let res = parse_inference_response(Ok(response)).await.unwrap();

    println!("{:?}", res.1);
}
