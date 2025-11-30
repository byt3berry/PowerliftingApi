use serde::Deserialize;
use types::prelude::*;

use crate::types::{Country, Federation};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct MeetData {
    #[serde(rename(deserialize = "Federation"))]
    #[serde(default)]
    pub federation: Federation,

    #[serde(rename(deserialize = "MeetCountry"))]
    #[serde(default)]
    pub country: Country,

    #[serde(rename(deserialize = "MeetState"))]
    #[serde(default)]
    pub state: String,

    #[serde(rename(deserialize = "MeetTown"))]
    #[serde(default)]
    pub town: String,

    #[serde(rename(deserialize = "MeetName"))]
    #[serde(default)]
    pub name: String,
}

impl From<MeetData> for MeetDataDto {
    fn from(value: MeetData) -> Self {
        Self {
            name: value.name,
            federation: value.federation.into(),
            country: value.country.into(),
            state: value.state,
            town: value.town,
        }
    }
}
