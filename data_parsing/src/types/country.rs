use serde::Deserialize;
use strum_macros::EnumIter;
use types::prelude::CountryDto;

#[derive(Copy, Clone, Debug, Default, Deserialize, EnumIter, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Country {
    #[strum(to_string = "France")]
    #[serde(rename(deserialize = "France"))]
    FRANCE,

    #[strum(to_string = "Other")]
    #[serde(other)]
    #[default]
    OTHER,
}

impl From<Country> for CountryDto {
    fn from(value: Country) -> Self {
        match value {
            Country::FRANCE => Self::FRANCE,
            Country::OTHER => Self::OTHER,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_test::{assert_de_tokens, Token};

    use super::Country;

    #[rstest]
    #[case("France", Country::FRANCE)]
    #[case("Some value", Country::OTHER)]
    fn test_deserialize(
        #[case] input: &'static str,
        #[case] expected: Country
    ) {
        assert_de_tokens(
            &expected, 
            &[Token::UnitVariant { name: "Country", variant: input }]
        );
    }
}

