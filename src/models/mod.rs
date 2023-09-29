use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QueryRequest {
    pub query: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmbeddingResponse {
    pub embedding: Vec<f32>,
}
