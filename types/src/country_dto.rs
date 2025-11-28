use crate::prelude::*;

#[derive(Copy, Clone, Debug, Default, Deserialize, EnumIter, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CountryDto {
    #[strum(to_string = "France")]
    #[serde(rename(deserialize = "France"))]
    FRANCE,

    #[strum(to_string = "Other")]
    #[serde(other)]
    #[default]
    OTHER,
}

impl Matches for CountryDto {
    fn matches(&self, _: &Self) -> bool {
        true
    }
}

impl MatchesQuery for CountryDto {
    fn matches_query(&self, _: &Query) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_test::{assert_de_tokens, Token};

    use super::CountryDto;

    #[rstest]
    #[case("France", CountryDto::FRANCE)]
    #[case("Some value", CountryDto::OTHER)]
    fn test_deserialize(
        #[case] input: &'static str,
        #[case] expected: CountryDto
    ) {
        assert_de_tokens(
            &expected, 
            &[Token::UnitVariant { name: "Country", variant: input }]
        );
    }
}

