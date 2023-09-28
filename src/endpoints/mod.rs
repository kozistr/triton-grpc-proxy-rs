pub mod embedding;

use actix_web::{post, HttpResponse, Responder};

use crate::endpoints::embedding::get_embedding;
use crate::models::{EmbeddingResponse, QueryRequest};

#[post("/v1/embedding")]
pub async fn get_v1_embedding(body: String) -> impl Responder {
    let requests: &Result<Vec<QueryRequest>, serde_json::Error> = &serde_json::from_str(&body);

    let requests: &Vec<QueryRequest> = match requests {
        Ok(req) => req,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let embeddings = get_embedding(
        requests
            .iter()
            .map(|req: &QueryRequest| req.query.clone())
            .collect(),
    )
    .await;

    let responses: Vec<EmbeddingResponse> = embeddings
        .iter()
        .map(|embedding| EmbeddingResponse { embedding: embedding.to_vec() })
        .collect();

    let response_string: String = serde_json::to_string(&responses).unwrap();

    HttpResponse::Ok().body(response_string)
}
