#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use ntex::web::{get, App, HttpResponse, HttpServer, Responder};

use crate::constants::{SERVER_PORT, TRITON_SERVER_URL};
use crate::endpoints::get_v1_embedding;

mod constants;
mod endpoints;
mod models;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let client: triton_client::Client = triton_client::Client::new(TRITON_SERVER_URL, None)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .state(client.clone())
            .service(health_check)
            .service(get_v1_embedding)
    })
    .bind(("127.0.0.1", SERVER_PORT))?
    .run()
    .await
}
