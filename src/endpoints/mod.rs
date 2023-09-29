pub mod embedding;

use actix_web::{post, HttpResponse, Responder};
use serde_json::from_str;

use crate::endpoints::embedding::get_embedding;
use crate::models::{EmbeddingResponse, QueryRequest};

#[post("/v1/embedding")]
pub async fn get_v1_embedding(body: String) -> impl Responder {
    let requests: Result<Vec<QueryRequest>, _> = from_str(&body);

    let queries: Vec<String> = match requests {
        Ok(req) => req.into_iter().map(|r: QueryRequest| r.query).collect(),
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let embeddings = get_embedding(queries).await;

    let responses: Vec<EmbeddingResponse> = embeddings
        .axis_iter(ndarray::Axis(0))
        .map(|row| EmbeddingResponse { embedding: row.to_vec() })
        .collect();

    match serde_json::to_string(&responses) {
        Ok(response_string) => HttpResponse::Ok().body(response_string),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
