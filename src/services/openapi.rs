use std::sync::Arc;

use ntex::util::Bytes;
use ntex::web::types::{Path, State};
use ntex::web::ServiceConfig;
use ntex::{http, web};
use utoipa::OpenApi;

use super::{embedding, metrics};
use crate::error::HttpError;
use crate::models::{EmbeddingResponse, QueryRequest};

#[derive(OpenApi)]
#[openapi(
    paths(embedding::get_embeddings, metrics::get_metrics),
    components(schemas(EmbeddingResponse, QueryRequest, HttpError))
)]
pub(crate) struct ApiDoc;

#[web::get("/{tail}*")]
async fn get_swagger(
    tail: Path<String>,
    openapi_conf: State<Arc<utoipa_swagger_ui::Config<'static>>>,
) -> Result<web::HttpResponse, HttpError> {
    if tail.as_ref() == "swagger.json" {
        let spec = ApiDoc::openapi().to_json().map_err(|err| HttpError {
            status: http::StatusCode::INTERNAL_SERVER_ERROR,
            msg: format!("Error generating OpenAPI spec: {}", err),
        })?;
        return Ok(web::HttpResponse::Ok().content_type("application/json").body(spec));
    }
    let conf = openapi_conf.as_ref().clone();
    match utoipa_swagger_ui::serve(&tail, conf.into()).map_err(|err| HttpError {
        msg: format!("Error serving Swagger UI: {}", err),
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
    })? {
        None => Err(HttpError {
            status: http::StatusCode::NOT_FOUND,
            msg: format!("path not found: {}", tail),
        }),
        Some(file) => Ok({
            let bytes = Bytes::from(file.bytes.to_vec());
            web::HttpResponse::Ok().content_type(file.content_type).body(bytes)
        }),
    }
}

pub fn ntex_config(config: &mut ServiceConfig) {
    let swagger_config =
        Arc::new(utoipa_swagger_ui::Config::new(["/explorer/swagger.json"]).use_base_layout());
    config.service(web::scope("/explorer/").state(swagger_config).service(get_swagger));
}
