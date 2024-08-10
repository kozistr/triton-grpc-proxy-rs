use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Clone, ToSchema)]
pub struct QueryRequest {
    pub query: String,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct EmbeddingResponse {
    pub embedding: Vec<f32>,
}
