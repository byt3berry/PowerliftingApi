use serde::Deserialize;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, Deserialize, Eq, EnumIter, PartialEq)]
pub enum Sex {
    /// A gender-neutral title, including non-binary lifters.
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
