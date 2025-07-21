use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq)]
pub enum Division {
    #[serde(rename(deserialize = "Open"))]
    Open,

    #[serde(rename(deserialize = "Guest"))]
    #[serde(rename(deserialize = "Hors Match"))]
    #[serde(rename(deserialize = "Prime Time"))]
    G,

    #[serde(rename(deserialize = "Cadet"))]
    Cadet,

    #[serde(rename(deserialize = "Elite"))]
    Elite,

    #[serde(rename(deserialize = "Juniors"))]
    #[serde(rename(deserialize = "Jeunes"))]
    Juniors,

    #[serde(rename(deserialize = "Sub-Juniors"))]
    #[serde(rename(deserialize = "Subjunior/Junior"))]
    SubJuniors,

    #[serde(rename(deserialize = "Masters 1"))]
    Masters1,

    #[serde(rename(deserialize = "Masters 2"))]
    Masters2,

    #[serde(rename(deserialize = "Masters 3"))]
    Masters3,

    #[serde(rename(deserialize = "Masters 4"))]
    Masters4,

    #[serde(rename(deserialize = "Masters"))]
    Masters,

    #[serde(rename(deserialize = "Seniors"))]
    #[serde(rename(deserialize = "Senior/Master"))]
    Seniors,

    #[default]
    None,

}
