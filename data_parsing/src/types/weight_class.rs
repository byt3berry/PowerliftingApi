use anyhow::Result;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use types::prelude::WeightClassDto;
use std::fmt::{self, Display};
use std::str::FromStr;

use crate::types::weight::Weight;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeightClass {
    UnderOrEqual(Weight),
    Over(Weight),
}

impl From<WeightClass> for WeightClassDto {
    fn from(value: WeightClass) -> Self {
        match value {
            WeightClass::UnderOrEqual(weight) => Self::UnderOrEqual(weight.into()),
            WeightClass::Over(weight) => Self::Over(weight.into()),
        }
    }
}

impl FromStr for WeightClass {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(v) = s.strip_suffix('+') {
            v.parse::<Weight>().map(Self::Over)
        } else {
            s.parse::<Weight>().map(Self::UnderOrEqual)
        }
    }
}

impl Display for WeightClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnderOrEqual(weight) => write!(f, "{weight}"),
            Self::Over(weight) => write!(f, "+{weight}"),
        }
    }
}

struct WeightClassVisitor;

impl Visitor<'_> for WeightClassVisitor {
    type Value = WeightClass;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid weight class")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<WeightClass, E> {
        WeightClass::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for WeightClass {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(WeightClassVisitor)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::WeightClass;

    #[rstest]
    #[case("43", WeightClass::UnderOrEqual(43.into()))]
    #[case("44", WeightClass::UnderOrEqual(44.into()))]
    #[case("47", WeightClass::UnderOrEqual(47.into()))]
    #[case("48", WeightClass::UnderOrEqual(48.into()))]
    #[case("52", WeightClass::UnderOrEqual(52.into()))]
    #[case("53", WeightClass::UnderOrEqual(53.into()))]
    #[case("56", WeightClass::UnderOrEqual(56.into()))]
    #[case("57", WeightClass::UnderOrEqual(57.into()))]
    #[case("59", WeightClass::UnderOrEqual(59.into()))]
    #[case("60", WeightClass::UnderOrEqual(60.into()))]
    #[case("63", WeightClass::UnderOrEqual(63.into()))]
    #[case("66", WeightClass::UnderOrEqual(66.into()))]
    #[case("69", WeightClass::UnderOrEqual(69.into()))]
    #[case("72", WeightClass::UnderOrEqual(72.into()))]
    #[case("74", WeightClass::UnderOrEqual(74.into()))]
    #[case("76", WeightClass::UnderOrEqual(76.into()))]
    #[case("83", WeightClass::UnderOrEqual(83.into()))]
    #[case("84", WeightClass::UnderOrEqual(84.into()))]
    #[case("93", WeightClass::UnderOrEqual(93.into()))]
    #[case("105", WeightClass::UnderOrEqual(105.into()))]
    #[case("120", WeightClass::UnderOrEqual(120.into()))]
    #[case("63+", WeightClass::Over(63.into()))]
    #[case("83+", WeightClass::Over(83.into()))]
    #[case("84+", WeightClass::Over(84.into()))]
    #[case("105+", WeightClass::Over(105.into()))]
    #[case("120+", WeightClass::Over(120.into()))]
    fn test_deserialize(
        #[case] input: &str,
        #[case] expected: WeightClass,
    ) {
        let result: Result<WeightClass> = input.parse::<WeightClass>();

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case(WeightClass::UnderOrEqual(83.into()), "83".to_string())]
    #[case(WeightClass::Over(120.into()), "+120".to_string())]
    fn test_display(
        #[case] input: WeightClass,
        #[case] expected: String
    ) {
        let result: String = format!("{}", input);

        assert_eq!(expected, result);
    }
}
