use serde::Deserialize;
use strum_macros::{Display, EnumIter};

use crate::{Matches, MatchesQuery, Query};

#[derive(Copy, Clone, Debug, Deserialize, Default, Display, Eq, PartialEq, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum FederationDto {
    #[strum(to_string = "FFForce")]
    #[serde(rename(deserialize = "FFForce"))]
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
    #[serde(other)]
    #[default]
    OTHER,
}

impl Matches for FederationDto {
    fn matches(&self, other: &Self) -> bool {
        self.eq(&other)
    }
}

impl MatchesQuery for FederationDto {
    fn matches_query(&self, query: &Query) -> bool {
        self.matches(&query.federation_choice)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_test::{assert_de_tokens, Token};

    use super::FederationDto;

    #[rstest]
    #[case("FFForce", FederationDto::FFForce)]
    #[case("EPF", FederationDto::EPF)]
    #[case("IPF", FederationDto::IPF)]
    #[case("FFHMFAC", FederationDto::FFHMFAC)]
    #[case("Some federation", FederationDto::OTHER)]
    fn test_deserialize(
        #[case] input: &'static str,
        #[case] expected: FederationDto,
    ) {
        assert_de_tokens(
            &expected, 
            &[Token::UnitVariant { name: "Federation", variant: input }]
        );
    }
}
