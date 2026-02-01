use actix_web::{get, HttpResponse, Responder};

use frontend::endpoints::root_page;

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok()
        .body(root_page())
}
