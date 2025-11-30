use serde::Deserialize;
use strum_macros::{Display, EnumIter};
use types::prelude::SexDto;

#[derive(Clone, Copy, Debug, Display, Deserialize, Eq, EnumIter, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    #[strum(to_string = "Men")]
    #[serde(rename(deserialize = "M"))]
    #[serde(rename(deserialize = "Men"))]
    M,

    #[strum(to_string = "Women")]
    #[serde(rename(deserialize = "F"))]
    #[serde(rename(deserialize = "Women"))]
    F,
}

impl From<Sex> for SexDto {
    fn from(value: Sex) -> Self {
        match value {
            Sex::M => Self::M,
            Sex::F => Self::F,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_test::{assert_de_tokens, Token};

    use super::Sex;

    #[rstest]
    #[case("M", Sex::M)]
    #[case("F", Sex::F)]
    fn test_deserialize(
        #[case] input: &'static str,
        #[case] expected: Sex
    ) {
        assert_de_tokens(
            &expected, 
            &[Token::UnitVariant { name: "Sex", variant: input }]
        );
    }

    #[rstest]
    #[case(Sex::F, Sex::F, true)]
    #[case(Sex::F, Sex::M, false)]
    #[case(Sex::M, Sex::F, false)]
    #[case(Sex::M, Sex::M, true)]
    fn test_eq(
        #[case] input1: Sex,
        #[case] input2: Sex,
        #[case] expected: bool,
    ) {
        assert_eq!(expected, input1.eq(&input2))
    }
}
