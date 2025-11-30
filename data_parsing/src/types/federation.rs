use serde::Deserialize;
use strum_macros::{Display, EnumIter};
use types::prelude::FederationDto;

#[derive(Copy, Clone, Debug, Deserialize, Default, Display, Eq, PartialEq, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum Federation {
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

impl From<Federation> for FederationDto {
    fn from(value: Federation) -> Self {
        match value {
            Federation::FFForce => Self::FFForce,
            Federation::EPF => Self::EPF,
            Federation::IPF => Self::IPF,
            Federation::FFHMFAC => Self::FFHMFAC,
            Federation::OTHER => Self::OTHER,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_test::{assert_de_tokens, Token};

    use super::Federation;

    #[rstest]
    #[case("FFForce", Federation::FFForce)]
    #[case("EPF", Federation::EPF)]
    #[case("IPF", Federation::IPF)]
    #[case("FFHMFAC", Federation::FFHMFAC)]
    #[case("Some federation", Federation::OTHER)]
    fn test_deserialize(
        #[case] input: &'static str,
        #[case] expected: Federation,
    ) {
        assert_de_tokens(
            &expected, 
            &[Token::UnitVariant { name: "Federation", variant: input }]
        );
    }
}
