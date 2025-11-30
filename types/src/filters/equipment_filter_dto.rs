use serde::Deserialize;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EquipmentFilterDto {
    #[strum(to_string = "Any")]
    #[serde(rename(deserialize = "Any"))]
    Any,

    #[strum(to_string = "Raw")]
    #[serde(rename(deserialize = "Raw"))]
    Raw,

    #[strum(to_string = "Wraps")]
    #[serde(rename(deserialize = "Wraps"))]
    Wraps,

    #[strum(to_string = "Single-ply")]
    #[serde(rename(deserialize = "Single"))]
    #[serde(rename(deserialize = "Single-ply"))]
    Single,

    #[strum(to_string = "Multi-ply")]
    #[serde(rename(deserialize = "Multi"))]
    #[serde(rename(deserialize = "Multi-ply"))]
    Multi,

    #[strum(to_string = "Straps")]
    #[serde(rename(deserialize = "Straps"))]
    Straps,

    #[strum(to_string = "Sleeves")]
    #[serde(rename(deserialize = "Sleeves"))]
    Sleeves,

    #[strum(to_string = "Bare")]
    #[serde(rename(deserialize = "Bare"))]
    Bare,

    #[strum(to_string = "Unlimited")]
    Unlimited,
}
