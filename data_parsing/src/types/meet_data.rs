use chrono::NaiveDate;
use serde::Deserialize;

use types::prelude::MeetDataDto;

use crate::types::{Country, Federation};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct MeetData {
    #[serde(rename(deserialize = "Federation"))]
    pub federation: Federation,

    #[serde(rename(deserialize = "Date"))]
    pub date: NaiveDate,

    #[serde(rename(deserialize = "MeetCountry"))]
    pub country: Country,

    #[serde(rename(deserialize = "MeetState"))]
    pub state: String,

    #[serde(rename(deserialize = "MeetTown"))]
    pub town: String,

    #[serde(rename(deserialize = "MeetName"))]
    pub name: String,
}

impl From<MeetData> for MeetDataDto {
    fn from(value: MeetData) -> Self {
        Self {
            name: value.name,
            date: value.date,
            federation: value.federation.into(),
            country: value.country.into(),
            state: value.state,
            town: value.town,
        }
    }
}
