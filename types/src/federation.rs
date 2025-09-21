use serde::Deserialize;
use strum_macros::{Display, EnumIter};

#[derive(Copy, Clone, Debug, Deserialize, Default, Display, Eq, PartialEq, EnumIter)]
pub enum Federation {
    #[strum(to_string = "FFForce")]
    #[serde(rename(deserialize = "FFFORCE"))]
    FFForce,

    #[strum(to_string = "EPF")]
    #[serde(rename(deserialize = "EPF"))]
    EPF,

    #[strum(to_string = "IPF")]
    #[serde(rename(deserialize = "IPF"))]
    IPF,

    #[strum(to_string = "FFHMFAC")]
    #[serde(rename(deserialize = "FFHMFAC"))]
    FFHMFAC,

    #[strum(to_string = "Other")]
    #[default]
    OTHER,
}
