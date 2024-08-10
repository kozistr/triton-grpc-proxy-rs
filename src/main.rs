#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use envconfig::Envconfig;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use ntex::web::middleware::Logger;
use ntex::web::{get, App, HttpResponse, HttpServer, Responder};

mod configs;
mod error;
mod models;
mod services;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let config: configs::Config = configs::Config::init_from_env().unwrap();

    let proxy_server_port: u16 = config.server_port;

    let client: triton_client::Client = triton_client::Client::new(
        format!("{}:{}", config.triton_server_url, config.triton_server_grpc_port),
        None,
    )
    .await
    .unwrap();

    let prom_builder: PrometheusBuilder = services::prometheus::prometheus_builer().unwrap();
    let prom_handler: PrometheusHandle =
        prom_builder.install_recorder().expect("failed to intall recorder");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .state(client.clone())
            .state(config.clone())
            .state(prom_handler.clone())
            .service(health_check)
            .service(services::metrics::get_metrics)
            .service(services::embedding::get_embeddings)
            .configure(services::openapi::ntex_config)
            .configure(services::embedding::ntex_config)
            .configure(services::metrics::ntex_config)
    })
    .bind(("0.0.0.0", proxy_server_port))?
    .run()
    .await
}
