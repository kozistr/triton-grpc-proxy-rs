pub mod embedding;
pub mod metrics;
pub mod prometheus;

pub use crate::endpoints::embedding::get_embeddings;
pub use crate::endpoints::metrics::get_metrics;
