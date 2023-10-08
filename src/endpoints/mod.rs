pub mod embedding;

use ntex::web::{post, Error, HttpResponse};
use serde_json::from_str;

use crate::endpoints::embedding::get_embedding;
use crate::models::{EmbeddingResponse, QueryRequest};

#[post("/v1/embedding")]
pub async fn get_v1_embedding(body: String) -> Result<HttpResponse, Error> {
    let requests: Result<Vec<QueryRequest>, _> = from_str(&body);

    let queries: Vec<String> = match requests {
        Ok(req) => req.into_iter().map(|r: QueryRequest| r.query).collect(),
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };

    let responses: Vec<EmbeddingResponse> = get_embedding(queries).await;

    Ok(HttpResponse::Ok().json(&responses))
}
