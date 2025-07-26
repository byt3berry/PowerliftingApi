use serde::Deserialize;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, Deserialize, Eq, EnumIter)]
pub enum Sex {
    #[strum(to_string = "All")]
    #[serde(rename(deserialize = "All"))]
    All,

    #[strum(to_string = "Men")]
    #[serde(rename(deserialize = "M"))]
    #[serde(rename(deserialize = "Men"))]
    M,

    #[strum(to_string = "Women")]
    #[serde(rename(deserialize = "F"))]
    #[serde(rename(deserialize = "Women"))]
    F,
}

impl PartialEq for Sex {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::All, _) => true,
            (Self::M, Self::M) => true,
            (Self::F, Self::F) => true,
            _ => false,
        }
    }
}
