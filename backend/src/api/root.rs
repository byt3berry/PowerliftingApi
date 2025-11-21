use actix_web::{get, HttpResponse, Responder};

use frontend::api::root_page;

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok()
        .body(root_page())
}
