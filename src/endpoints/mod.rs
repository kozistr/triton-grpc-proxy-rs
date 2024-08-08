pub mod embedding;

use ntex::web::types::{Json, State};
use ntex::web::{post, Error, HttpResponse};

use crate::configs::Config;
use crate::endpoints::embedding::get_embeddings_from_triton_server;
use crate::models::{EmbeddingResponse, QueryRequest};

#[post("/v1/embedding")]
pub async fn get_embeddings(
    requests: Json<Vec<QueryRequest>>,
    client: State<triton_client::Client>,
    config: State<Config>,
) -> Result<HttpResponse, Error> {
    let queries: Vec<&str> = requests.iter().map(|req: &QueryRequest| req.query.as_str()).collect();

    let responses: Vec<EmbeddingResponse> =
        get_embeddings_from_triton_server(&queries, &client, &config).await;

    Ok(HttpResponse::Ok().json(&responses))
}
