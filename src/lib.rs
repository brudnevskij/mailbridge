pub mod configuration;
pub mod routes;
pub mod startup;

use std::net::TcpListener;

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder,
    dev::Server,
    web::{self, Form},
};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("health_check", web::get().to(health_check))
            .route("subscribe", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
