use serde::Deserialize;
use types::Country;
use types::Federation;

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
