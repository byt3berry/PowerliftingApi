use serde::Deserialize;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, EnumIter)]
pub enum Equipment {
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

    #[strum(to_string = "Unlimited")]
    Unlimited,
}

impl PartialEq for Equipment {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Any, _) => true,
            (Equipment::Raw, Equipment::Raw) => true,
            (Equipment::Wraps, Equipment::Wraps) => true,
            (Equipment::Single, Equipment::Single) => true,
            (Equipment::Multi, Equipment::Multi) => true,
            (Equipment::Straps, Equipment::Straps) => true,
            (Equipment::Unlimited, Equipment::Unlimited) => true,
            _ => false,
        }
    }
}
