use actix_web::{HttpResponse, web::Form};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
