pub mod embedding;

use actix_web::{post, HttpResponse, Responder};

use crate::endpoints::embedding::get_embedding;
use crate::models::{EmbeddingResponse, QueryRequest};

#[post("/v1/embedding")]
pub async fn get_v1_embedding(body: String) -> impl Responder {
    let requests: &Vec<QueryRequest> = match &serde_json::from_str(&body) {
        Ok(req) => req,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    for req in requests {
        println!("{:?}", req.query);
    }

    let responses: Vec<EmbeddingResponse> = vec![];
    for _ in 0..requests.len() as i64 {
        let response: EmbeddingResponse = EmbeddingResponse { embedding: vec![0.0f32; 2048] };
        responses.push(response)
    }

    let response_string: String = serde_json::to_string(&responses)?;

    HttpResponse::Ok().body(response_string)

    // get_embedding(
    //     requests
    //         .iter()
    //         .map(|req: &QueryRequest| req.query)
    //         .collect()
    // )
}
