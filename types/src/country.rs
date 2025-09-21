use serde::Deserialize;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Default, Deserialize, EnumIter, Eq, PartialEq)]
pub enum Country {
    #[strum(to_string = "France")]
    #[serde(rename(deserialize = "France"))]
    FRANCE,

    #[strum(to_string = "Other")]
    #[default]
    OTHER,
}
