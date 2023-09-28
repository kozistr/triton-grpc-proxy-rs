use actix_web::{App, HttpServer};

use crate::endpoints::get_v1_embedding;

mod endpoints;
mod models;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| App::new().service(get_v1_embedding))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
