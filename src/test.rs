use std::collections::HashMap;

use ndarray::{Array, Array1, Array2};
use triton_client::inference::model_infer_request::{InferInputTensor, InferRequestedOutputTensor};
use triton_client::inference::ModelInferRequest;

fn serialize_byte_string(queries: Vec<Vec<u8>>) -> Vec<u8> {
    queries
        .iter()
        .flat_map(|query: &Vec<u8>| {
            let len_bytes: Vec<u8> = (query.len() as u32).to_le_bytes().to_vec();
            len_bytes.into_iter().chain(query.iter().copied())
        })
        .collect::<Vec<u8>>()
}

async fn test() -> Array2<f32> {
    let client: triton_client::Client = triton_client::Client::new("http://127.0.0.1:8001", None)
        .await
        .unwrap();

    let queries: Vec<Vec<u8>> = vec![
        b"asdf".to_vec(),
        b"asdf asdf".to_vec(),
        b"asdf asdf asdf".to_vec(),
        b"asdf asdf asdf asdf".to_vec(),
    ];
    let batch_size: usize = queries.len();

    let inputs: Vec<InferInputTensor> = vec![InferInputTensor {
        name: "text".into(),
        shape: vec![queries.len() as i64, 1],
        parameters: HashMap::new(),
        datatype: "BYTES".into(),
        contents: None,
    }];

    let flatten_queries: Vec<u8> = serialize_byte_string(queries);

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
        raw_input_contents: vec![flatten_queries],
    };

    let response: triton_client::inference::ModelInferResponse =
        client.model_infer(request).await.unwrap();

    let vectors = response
        .raw_output_contents
        .iter()
        .map(|r: &Vec<u8>| {
            let e: &[f32] = bytemuck::cast_slice::<u8, f32>(r.as_slice());
            Array::from_vec(e.to_vec())
        })
        .collect::<Vec<Array1<f32>>>();

    vectors[0].clone().into_shape((batch_size, 2048)).unwrap()
}

#[tokio::main]
async fn main() {
    let vectors = test().await;

    let vectors: Vec<Vec<f32>> = vectors
        .axis_iter(ndarray::Axis(0))
        .map(|row| row.to_vec())
        .collect();

    println!("{:?}", vectors);
}
