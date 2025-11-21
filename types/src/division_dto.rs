use serde::Deserialize;
use strum_macros::{Display, EnumIter};

use crate::{Matches, MatchesQuery, Query};

#[derive(Clone, Copy, Debug, Display, Deserialize, Eq, EnumIter, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DivisionDto {
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

impl Matches for DivisionDto {
    fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Any, _) => true,
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
            (Self::Seniors, Self::Seniors) => true,
            _ => false,
        }
    }
}

impl MatchesQuery for DivisionDto {
    fn matches_query(&self, other: &Query) -> bool {
        self.matches(&other.division_choice)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_test::{assert_de_tokens, Token};

    use super::DivisionDto;

    #[rstest]
    #[case("Cadet", DivisionDto::Cadet)]
    #[case("Elite", DivisionDto::Elite)]
    #[case("Hors Match", DivisionDto::G)]
    #[case("Jeunes", DivisionDto::Juniors)]
    #[case("Juniors", DivisionDto::Juniors)]
    #[case("Masters", DivisionDto::Masters)]
    #[case("Masters 1", DivisionDto::Masters1)]
    #[case("Masters 2", DivisionDto::Masters2)]
    #[case("Masters 3", DivisionDto::Masters3)]
    #[case("Masters 4", DivisionDto::Masters4)]
    #[case("Open", DivisionDto::Open)]
    #[case("Prime Time", DivisionDto::G)]
    #[case("Senior/Master", DivisionDto::Seniors)]
    #[case("Seniors", DivisionDto::Seniors)]
    #[case("Sub-Juniors", DivisionDto::SubJuniors)]
    fn test_deserialize(
        #[case] input: &'static str,
        #[case] expected: DivisionDto,
    ) {
        assert_de_tokens(
            &expected, 
            &[Token::UnitVariant { name: "Division", variant: input }]
        );
    }
}
