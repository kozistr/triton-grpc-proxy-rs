pub mod embedding;

use actix_web::{post, HttpResponse, Responder};
use ndarray::{Array2, Axis};
use serde_json::{from_str, to_string};

use crate::endpoints::embedding::get_embedding;
use crate::models::{EmbeddingResponse, QueryRequest};

#[post("/v1/embedding")]
pub async fn get_v1_embedding(body: String) -> impl Responder {
    let requests: Result<Vec<QueryRequest>, _> = from_str(&body);

    let queries: Vec<String> = match requests {
        Ok(req) => req.into_iter().map(|r: QueryRequest| r.query).collect(),
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let embeddings: Array2<f32> = get_embedding(queries).await;

    let responses: Vec<EmbeddingResponse> = embeddings
        .axis_iter(Axis(0))
        .map(|row| EmbeddingResponse { embedding: row.to_vec() })
        .collect();

    match to_string(&responses) {
        Ok(response_string) => HttpResponse::Ok().body(response_string),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
