use crate::prelude::*;

#[derive(Clone, Copy, Debug, Display, Deserialize, Eq, EnumIter, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SexDto {
    #[strum(to_string = "Men")]
    #[serde(rename(deserialize = "M"))]
    #[serde(rename(deserialize = "Men"))]
    M,

    #[strum(to_string = "Women")]
    #[serde(rename(deserialize = "F"))]
    #[serde(rename(deserialize = "Women"))]
    F,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_test::{assert_de_tokens, Token};

    use super::SexDto;

    #[rstest]
    #[case("M", SexDto::M)]
    #[case("F", SexDto::F)]
    fn test_deserialize(
        #[case] input: &'static str,
        #[case] expected: SexDto
    ) {
        assert_de_tokens(
            &expected, 
            &[Token::UnitVariant { name: "SexDto", variant: input }]
        );
    }

    #[rstest]
    #[case(SexDto::F, SexDto::F, true)]
    #[case(SexDto::F, SexDto::M, false)]
    #[case(SexDto::M, SexDto::F, false)]
    #[case(SexDto::M, SexDto::M, true)]
    fn test_eq(
        #[case] input1: SexDto,
        #[case] input2: SexDto,
        #[case] expected: bool,
    ) {
        assert_eq!(expected, input1.eq(&input2))
    }
}
