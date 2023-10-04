pub mod embedding;

use actix_web::{post, HttpResponse, Responder};
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

    let responses: Vec<EmbeddingResponse> = get_embedding(queries).await;

    match to_string(&responses) {
        Ok(response_string) => HttpResponse::Ok().body(response_string),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
