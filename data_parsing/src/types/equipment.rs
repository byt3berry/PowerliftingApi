use serde::Deserialize;
use strum_macros::{Display, EnumIter};
use types::prelude::EquipmentDto;

#[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Equipment {
    #[strum(to_string = "Any")]
    #[serde(rename(deserialize = "Any"))]
    Any,

    #[strum(to_string = "Raw")]
    #[serde(rename(deserialize = "Raw"))]
    Raw,

    #[strum(to_string = "Wraps")]
    #[serde(rename(deserialize = "Wraps"))]
    Wraps,

    #[strum(to_string = "Single-ply")]
    #[serde(rename(deserialize = "Single"))]
    #[serde(rename(deserialize = "Single-ply"))]
    Single,

    #[strum(to_string = "Multi-ply")]
    #[serde(rename(deserialize = "Multi"))]
    #[serde(rename(deserialize = "Multi-ply"))]
    Multi,

    #[strum(to_string = "Straps")]
    #[serde(rename(deserialize = "Straps"))]
    Straps,

    #[strum(to_string = "Sleeves")]
    #[serde(rename(deserialize = "Sleeves"))]
    Sleeves,

    #[strum(to_string = "Bare")]
    #[serde(rename(deserialize = "Bare"))]
    Bare,

    #[strum(to_string = "Unlimited")]
    Unlimited,
}

impl From<Equipment> for EquipmentDto {
    fn from(value: Equipment) -> Self {
        match value {
            Equipment::Any => Self::Any,
            Equipment::Raw => Self::Raw,
            Equipment::Wraps => Self::Wraps,
            Equipment::Single => Self::Single,
            Equipment::Multi => Self::Multi,
            Equipment::Straps => Self::Straps,
            Equipment::Sleeves => Self::Sleeves,
            Equipment::Bare => Self::Bare,
            Equipment::Unlimited => Self::Unlimited,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_test::{assert_de_tokens, Token};

    use super::Equipment;

    #[rstest]
    #[case("Any", Equipment::Any)]
    #[case("Raw", Equipment::Raw)]
    #[case("Wraps", Equipment::Wraps)]
    #[case("Single", Equipment::Single)]
    #[case("Single-ply", Equipment::Single)]
    #[case("Multi", Equipment::Multi)]
    #[case("Multi-ply", Equipment::Multi)]
    #[case("Bare", Equipment::Bare)]
    #[case("Sleeves", Equipment::Sleeves)]
    #[case("Straps", Equipment::Straps)]
    fn test_deserialize(
        #[case] input: &'static str,
        #[case] expected: Equipment,
    ) {
        assert_de_tokens(
            &expected, 
            &[Token::UnitVariant { name: "Equipment", variant: input }]
        );
    }
}
