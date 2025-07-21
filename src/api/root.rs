use actix_web::{get, HttpResponse, Responder};
use maud::{html, Markup, DOCTYPE};

use crate::api::{body, head};

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok()
        .body(root_page())
}

fn root_page() -> Markup {
    html! {
        (DOCTYPE)
        (head())
        (body())
    }
}
