use serde::Deserialize;
use strum_macros::{Display, EnumIter};
use types::prelude::DivisionDto;

#[derive(Clone, Copy, Debug, Display, Deserialize, Eq, EnumIter, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Division {
    #[strum(to_string = "Any")]
    #[serde(rename(deserialize = "Any"))]
    Any,

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

impl From<Division> for DivisionDto {
    fn from(value: Division) -> Self {
        match value {
            Division::Any => Self::Any,
            Division::Open => Self::Open,
            Division::G => Self::G,
            Division::Cadet => Self::Cadet,
            Division::Elite => Self::Elite,
            Division::SubJuniors => Self::SubJuniors,
            Division::Juniors => Self::Juniors,
            Division::Masters => Self::Masters,
            Division::Seniors => Self::Seniors,
            Division::Masters1 => Self::Masters1,
            Division::Masters2 => Self::Masters2,
            Division::Masters3 => Self::Masters3,
            Division::Masters4 => Self::Masters4,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_test::{assert_de_tokens, Token};

    use super::Division;

    #[rstest]
    #[case("Cadet", Division::Cadet)]
    #[case("Elite", Division::Elite)]
    #[case("Hors Match", Division::G)]
    #[case("Jeunes", Division::Juniors)]
    #[case("Juniors", Division::Juniors)]
    #[case("Masters", Division::Masters)]
    #[case("Masters 1", Division::Masters1)]
    #[case("Masters 2", Division::Masters2)]
    #[case("Masters 3", Division::Masters3)]
    #[case("Masters 4", Division::Masters4)]
    #[case("Open", Division::Open)]
    #[case("Prime Time", Division::G)]
    #[case("Senior/Master", Division::Seniors)]
    #[case("Seniors", Division::Seniors)]
    #[case("Sub-Juniors", Division::SubJuniors)]
    fn test_deserialize(
        #[case] input: &'static str,
        #[case] expected: Division,
    ) {
        assert_de_tokens(
            &expected, 
            &[Token::UnitVariant { name: "Division", variant: input }]
        );
    }
}
