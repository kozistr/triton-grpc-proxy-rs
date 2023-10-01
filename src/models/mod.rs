use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct QueryRequest {
    pub query: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct EmbeddingResponse {
    pub embedding: Vec<f32>,
}
