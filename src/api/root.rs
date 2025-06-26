use actix_web::{get, HttpResponse, Responder};
use maud::{html, Markup, DOCTYPE};

use crate::api::{head, input_div, result_div};

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok()
        .body(root_page())
}

fn root_page() -> Markup {
    html! {
        (DOCTYPE)
        (head())
        (input_div())
        (result_div())
    }
}
