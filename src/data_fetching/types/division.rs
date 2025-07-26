use serde::Deserialize;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, Deserialize, Eq, EnumIter)]
pub enum Division {
    #[strum(to_string = "All")]
    #[serde(rename(deserialize = "All"))]
    All,

    #[strum(to_string = "Open")]
    #[serde(rename(deserialize = "Open"))]
    Open,

    #[strum(to_string = "Guest")]
    #[serde(rename(deserialize = "Guest"))]
    #[serde(rename(deserialize = "Hors Match"))]
    #[serde(rename(deserialize = "Prime Time"))]
    G,

    #[strum(to_string = "Cadet")]
    #[serde(rename(deserialize = "Cadet"))]
    Cadet,

    #[strum(to_string = "Elite")]
    #[serde(rename(deserialize = "Elite"))]
    Elite,

    #[strum(to_string = "Sub-Juniors")]
    #[serde(rename(deserialize = "Sub-Juniors"))]
    #[serde(rename(deserialize = "Subjunior/Junior"))]
    SubJuniors,

    #[strum(to_string = "Juniors")]
    #[serde(rename(deserialize = "Juniors"))]
    #[serde(rename(deserialize = "Jeunes"))]
    Juniors,

    #[strum(to_string = "Masters")]
    #[serde(rename(deserialize = "Masters"))]
    Masters,

    #[strum(to_string = "Seniors")]
    #[serde(rename(deserialize = "Seniors"))]
    #[serde(rename(deserialize = "Senior/Master"))]
    Seniors,

    #[strum(to_string = "Masters 1")]
    #[serde(rename(deserialize = "Masters 1"))]
    Masters1,

    #[strum(to_string = "Masters 2")]
    #[serde(rename(deserialize = "Masters 2"))]
    Masters2,

    #[strum(to_string = "Masters 3")]
    #[serde(rename(deserialize = "Masters 3"))]
    Masters3,

    #[strum(to_string = "Masters 4")]
    #[serde(rename(deserialize = "Masters 4"))]
    Masters4,
}

impl PartialEq for Division {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::All, _) => true,
            (Self::Open, Self::Open) => true,
            (Self::G, Self::G) => true,
            (Self::Cadet, Self::Cadet) => true,
            (Self::Elite, Self::Elite) => true,
            (Self::SubJuniors, Self::SubJuniors) => true,
            (Self::Juniors, Self::Juniors) => true,
            (Self::Masters, Self::Masters | Self::Masters1 | Self::Masters2 | Self::Masters3 | Self::Masters4) => true,
            (Self::Masters1, Self::Masters1) => true,
            (Self::Masters2, Self::Masters2) => true,
            (Self::Masters3, Self::Masters3) => true,
            (Self::Masters4, Self::Masters4) => true,
            _ => false,
        }
    }
}
