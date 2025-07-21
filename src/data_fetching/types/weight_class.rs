use std::{fmt, num, str::FromStr};

use serde::{de::{self, Visitor}, Deserialize};

use crate::data_fetching::types::weight::Weight;

#[derive(Debug, Default, PartialEq)]
pub enum WeightClass {
    UnderOrEqual(Weight),
    Over(Weight),
    #[default]
    None,
}

impl FromStr for WeightClass {
    type Err = num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            println!("iciiii");
            return Ok(WeightClass::None);
        }

        if let Some(v) = s.strip_suffix('+') {
            v.parse::<Weight>().map(Self::Over)
        } else {
            s.parse::<Weight>().map(Self::UnderOrEqual)
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
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(WeightClassVisitor)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use pretty_assertions::assert_eq;

    use crate::data_fetching::types::{weight::Weight, weight_class::WeightClass};

    #[rstest]
    #[case("43", WeightClass::UnderOrEqual(Weight(43.)))]
    #[case("44", WeightClass::UnderOrEqual(Weight(44.)))]
    #[case("47", WeightClass::UnderOrEqual(Weight(47.)))]
    #[case("48", WeightClass::UnderOrEqual(Weight(48.)))]
    #[case("52", WeightClass::UnderOrEqual(Weight(52.)))]
    #[case("53", WeightClass::UnderOrEqual(Weight(53.)))]
    #[case("56", WeightClass::UnderOrEqual(Weight(56.)))]
    #[case("57", WeightClass::UnderOrEqual(Weight(57.)))]
    #[case("59", WeightClass::UnderOrEqual(Weight(59.)))]
    #[case("60", WeightClass::UnderOrEqual(Weight(60.)))]
    #[case("63", WeightClass::UnderOrEqual(Weight(63.)))]
    #[case("66", WeightClass::UnderOrEqual(Weight(66.)))]
    #[case("69", WeightClass::UnderOrEqual(Weight(69.)))]
    #[case("72", WeightClass::UnderOrEqual(Weight(72.)))]
    #[case("74", WeightClass::UnderOrEqual(Weight(74.)))]
    #[case("76", WeightClass::UnderOrEqual(Weight(76.)))]
    #[case("83", WeightClass::UnderOrEqual(Weight(83.)))]
    #[case("84", WeightClass::UnderOrEqual(Weight(84.)))]
    #[case("93", WeightClass::UnderOrEqual(Weight(93.)))]
    #[case("105", WeightClass::UnderOrEqual(Weight(105.)))]
    #[case("120", WeightClass::UnderOrEqual(Weight(120.)))]
    #[case("63+", WeightClass::Over(Weight(63.)))]
    #[case("83+", WeightClass::Over(Weight(83.)))]
    #[case("84+", WeightClass::Over(Weight(84.)))]
    #[case("105+", WeightClass::Over(Weight(105.)))]
    #[case("120+", WeightClass::Over(Weight(120.)))]
    fn test_deserialize(
        #[case] weight_class: &str,
        #[case] expected: WeightClass,
    ) {
        let weight_class: WeightClass = weight_class.parse::<WeightClass>().unwrap();

        assert_eq!(weight_class, expected);
    }
}
