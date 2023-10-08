// todo : move to docker secret or env wherever
pub const SERVER_PORT: u16 = 8080;
pub const TRITON_SERVER_URL: &str = "http://triton-server:8001";
pub const MODEL_VERSION: &str = "1";
pub const MODEL_NAME: &str = "model";
pub const INPUT_NAME: &str = "text";
pub const OUTPUT_NAME: &str = "embedding";
pub const V1_EMBEDDING_SIZE: usize = 2048;
