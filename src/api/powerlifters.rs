use actix_web::{post, HttpResponse, Responder};
use actix_web::web::{Data, Form};
use log::debug;
use maud::{html, Markup};
use serde::Deserialize;

use crate::data_fetching::POWERLIFTER_TABLE_HEADERS;
use crate::data_fetching::types::meet_entry::MeetEntry;
use crate::server::ServerData;

#[derive(Debug, Deserialize)]
struct PowerlifterForm {
    pub powerlifters: String,
}

#[post("/powerlifters")]
pub async fn powerlifters(form: Form<PowerlifterForm>, data: Data<ServerData>) -> impl Responder {
    debug!("form: {form:?}");
    let powerlifter_data: Vec<MeetEntry> = MeetEntry::from_string(&form.powerlifters);
    HttpResponse::Ok().body(build_table(powerlifter_data))
}


fn build_table(data: Vec<MeetEntry>) -> Markup {
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
