use actix_web::{post, web::Form, HttpResponse, Responder};
use log::debug;
use maud::{html, Markup};
use serde::Deserialize;

use crate::data_fetching::{LifterEntry, POWERLIFTER_TABLE_HEADERS};

#[derive(Debug, Deserialize)]
struct PowerlifterForm {
    pub powerlifters: String,
}

#[post("/powerlifters")]
pub async fn powerlifters(data: Form<PowerlifterForm>) -> impl Responder {
    debug!("data: {data:?}");
    let powerlifter_data: Vec<LifterEntry> = LifterEntry::from_string(&data.powerlifters);
    HttpResponse::Ok().body(build_table(powerlifter_data))
}


fn build_table(data: Vec<LifterEntry>) -> Markup {
    html! {
        table {

            tr {
                @for header in &POWERLIFTER_TABLE_HEADERS {
                    th { (header) }
                }
            }

            @for row in data {
                tr {
                    td { (row.name) }
                    @for _ in 0..&POWERLIFTER_TABLE_HEADERS.len()-1 {
                        td { }
                    }
                }
            }

        }
    }
}
