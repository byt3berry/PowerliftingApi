use serde::Deserialize;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, Deserialize, Eq, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    #[strum(to_string = "Any")]
    #[serde(rename(deserialize = "Any"))]
    Any,

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
            (Self::Any, _) => true,
            (_, Self::Any) => true,
            (Self::F, Self::F) => true,
            (Self::M, Self::M) => true,
            _ => false,
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
    #[case(Sex::Any, Sex::Any, true)]
    #[case(Sex::Any, Sex::F, true)]
    #[case(Sex::Any, Sex::M, true)]
    #[case(Sex::F, Sex::Any, true)]
    #[case(Sex::F, Sex::F, true)]
    #[case(Sex::F, Sex::M, false)]
    #[case(Sex::M, Sex::Any, true)]
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
