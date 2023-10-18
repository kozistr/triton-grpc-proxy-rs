pub mod embedding;

use ntex::web::types::{Json, State};
use ntex::web::{post, Error, HttpResponse};

use crate::configs::Config;
use crate::endpoints::embedding::get_embedding;
use crate::models::{EmbeddingResponse, QueryRequest};

#[post("/v1/embedding")]
pub async fn get_v1_embedding(
    requests: Json<Vec<QueryRequest>>,
    client: State<triton_client::Client>,
    config: State<Config>,
) -> Result<HttpResponse, Error> {
    let queries: Vec<String> =
        requests.into_inner().into_iter().map(|req: QueryRequest| req.query).collect();

    let responses: Vec<EmbeddingResponse> = get_embedding(queries, client, config).await;

    Ok(HttpResponse::Ok().json(&responses))
}
