use envconfig::Envconfig;

#[derive(Envconfig, Clone)]
pub struct Config {
    #[envconfig(from = "SERVER_PORT", default = "8080")]
    pub server_port: u16,

    #[envconfig(from = "TRITON_SERVER_URL", default = "http://triton-server")]
    pub triton_server_url: String,

    #[envconfig(from = "TRITON_SERVER_GRPC_PORT", default = "8001")]
    pub triton_server_grpc_port: u16,

    #[envconfig(from = "MODEL_VERSION", default = "1")]
    pub model_version: String,

    #[envconfig(from = "MODEL_NAME", default = "model")]
    pub model_name: String,

    #[envconfig(from = "INPUT_NAME", default = "text")]
    pub input_name: String,

    #[envconfig(from = "OUTPUT_NAME", default = "embedding")]
    pub output_name: String,

    #[envconfig(from = "EMBEDDING_SIZE", default = "2048")]
    pub embedding_size: usize,
}
