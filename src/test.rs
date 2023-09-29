use std::collections::HashMap;

use ndarray::{Array, Array1};
use triton_client::inference::model_infer_request::{InferInputTensor, InferRequestedOutputTensor};
use triton_client::inference::ModelInferRequest;

async fn test() {
    let client: triton_client::Client = triton_client::Client::new("http://127.0.0.1:8001", None)
        .await
        .unwrap();

    // let queries = vec![
    //     "asdf".to_owned().as_bytes().to_owned(),
    //     "asdf asdf".to_owned().as_bytes().to_owned(),
    //     // "asdf asdf".to_owned(),
    //     // "asdf asdf".to_owned(),
    // ];
    // let queries = vec![String::from("asdf").as_bytes().to_owned()];
    let queries: Vec<Vec<u8>> = vec![
        b"asdf"
            .to_vec()
            .iter()
            .map(|t| t.to_le_bytes().to_vec())
            .flatten()
            .collect::<Vec<u8>>(),
    ];

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
        raw_input_contents: queries,
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

    println!("{:?}", vectors);
}

#[tokio::main]
async fn main() {
    let q: Vec<u8> = b"asdf"
        .to_vec()
        .iter()
        .map(|t: &u8| t.to_le_bytes().to_vec())
        .flatten()
        .collect::<Vec<u8>>();
    println!("{:?}", q);

    test().await;
}
