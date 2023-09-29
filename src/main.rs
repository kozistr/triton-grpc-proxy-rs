use actix_web::{web, App, HttpResponse, HttpServer};
use mimalloc::MiMalloc;

use crate::constants::SERVER_PORT;
use crate::endpoints::get_v1_embedding;

mod constants;
mod endpoints;
mod models;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

async fn get_health_status() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("Ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "error");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(get_health_status))
            .service(get_v1_embedding)
    })
    .bind(("127.0.0.1", SERVER_PORT))?
    .run()
    .await
}
