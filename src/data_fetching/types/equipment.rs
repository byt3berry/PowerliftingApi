use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
pub enum Equipment {
    #[serde(rename(deserialize = "Raw"))]
    Raw,

    #[serde(rename(deserialize = "Wraps"))]
    Wraps,

    #[serde(rename(deserialize = "Single"))]
    #[serde(rename(deserialize = "Single-ply"))]
    Single,

    #[serde(rename(deserialize = "Multi"))]
    Multi,

    #[serde(rename(deserialize = "Straps"))]
    Straps,

    #[default]
    Unlimited,
}
