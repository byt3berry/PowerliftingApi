use actix_web::{post, web::Form, HttpResponse, Responder};
use maud::{html, Markup};
use serde::Deserialize;

use crate::data_fetching::{get_powerlifter_data, PowerlifterData, POWERLIFTER_TABLE_HEADERS};

#[derive(Debug, Deserialize)]
struct PowerlifterForm {
    pub powerlifters: String,
}

#[post("/powerlifters")]
pub async fn powerlifters(data: Form<PowerlifterForm>) -> impl Responder {
    println!("data: {data:?}");
    let powerlifter_data: Vec<PowerlifterData> = get_powerlifter_data(&data.powerlifters);
    HttpResponse::Ok().json(vec![1, 2, 3, 4, 5])
}


fn build_table(data: Vec<PowerlifterData>) -> Markup {
    html! {
        table {
            tr {
                @for header in &POWERLIFTER_TABLE_HEADERS {
                    th { (header) }
                }
            }
        }
    }
}
