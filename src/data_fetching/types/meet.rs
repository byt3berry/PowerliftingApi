use serde::Deserialize;

use crate::data_fetching::types::country::Country;
use crate::data_fetching::types::federation::Federation;

#[derive(Debug, Deserialize)]
pub struct Meet {
    #[serde(rename(deserialize = "Federation"))]
    #[serde(default)]
    pub federation: Federation,

    #[serde(rename(deserialize = "MeetCountry"))]
    #[serde(default)]
    pub country: Country,

    #[serde(rename(deserialize = "MeetName"))]
    pub name: String,
}
