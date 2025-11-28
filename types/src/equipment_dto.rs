use crate::prelude::*;

#[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EquipmentDto {
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

impl Matches for EquipmentDto {
    fn matches(&self, other: &Self) -> bool {
        Self::Any.eq(&other) || self.eq(other)
    }
}

impl MatchesQuery for EquipmentDto {
    fn matches_query(&self, query: &Query) -> bool {
        self.matches(&query.equipment_choice)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_test::{assert_de_tokens, Token};

    use super::EquipmentDto;

    #[rstest]
    #[case("Any", EquipmentDto::Any)]
    #[case("Raw", EquipmentDto::Raw)]
    #[case("Wraps", EquipmentDto::Wraps)]
    #[case("Single", EquipmentDto::Single)]
    #[case("Single-ply", EquipmentDto::Single)]
    #[case("Multi", EquipmentDto::Multi)]
    #[case("Multi-ply", EquipmentDto::Multi)]
    #[case("Bare", EquipmentDto::Bare)]
    #[case("Sleeves", EquipmentDto::Sleeves)]
    #[case("Straps", EquipmentDto::Straps)]
    fn test_deserialize(
        #[case] input: &'static str,
        #[case] expected: EquipmentDto,
    ) {
        assert_de_tokens(
            &expected, 
            &[Token::UnitVariant { name: "Equipment", variant: input }]
        );
    }
}
