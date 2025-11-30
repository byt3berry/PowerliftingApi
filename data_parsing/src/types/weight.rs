use anyhow::Result;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use types::prelude::WeightDto;
use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weight(pub f32);

impl From<Weight> for WeightDto {
    fn from(value: Weight) -> Self {
        Self(value.0)
    }
}

impl From<i64> for Weight {
    #[inline]
    fn from(i: i64) -> Self {
        Self(i as f32)
    }
}

impl From<u64> for Weight {
    #[inline]
    fn from(u: u64) -> Self {
        Self(u as f32)
    }
}

impl From<f64> for Weight {
    #[inline]
    fn from(f: f64) -> Self {
        if f.is_finite() {
            Self(f as f32)
        } else {
            Self(0.)
        }
    }
}

impl From<Weight> for f64 {
    #[inline]
    fn from(w: Weight) -> Self {
        Self::from(w.0)
    }
}

impl FromStr for Weight {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Self(0.))
        } else {
            Ok(Self::from(s.parse::<f64>()?))
        }
    }
}

impl Eq for Weight { }

impl Ord for Weight {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 {
            return Ordering::Less;
        }

        if self.0 > other.0 {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}

impl PartialOrd for Weight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for Weight {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(WeightVisitor)
    }
}

impl Weight {
    #[must_use]
    pub fn is_zero(self) -> bool {
        self == Self::from(0.)
    }
}

struct WeightVisitor;

impl Visitor<'_> for WeightVisitor {
    type Value = Weight;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a number or numeric string")
    }

    fn visit_i64<E: de::Error>(self, i: i64) -> Result<Self::Value, E> {
        Ok(Self::Value::from(i))
    }

    fn visit_u64<E: de::Error>(self, u: u64) -> Result<Self::Value, E> {
        Ok(Self::Value::from(u))
    }

    fn visit_f64<E: de::Error>(self, f: f64) -> Result<Self::Value, E> {
        Ok(Self::Value::from(f))
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Self::Value::from_str(v).map_err(E::custom)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::Comparison;
    use rstest::rstest;
    use std::cmp::Ordering;
    use std::str::FromStr;

    use super::Weight;

    #[rstest]
    #[case("1", Weight(1.))]
    #[case("1.5", Weight(1.5))]
    #[case("-1", Weight(-1.))]
    #[case("-1.5", Weight(-1.5))]
    fn test_deserialize(
        #[case] input: &str,
        #[case] expected: Weight,
    ) {
        let result: Result<Weight> = input.parse::<Weight>();

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case("Test")]
    fn test_deserialize_fail(
        #[case] input: &str
    ) {
        let result: Result<Weight> = input.parse::<Weight>();

        assert!(result.is_err(), "{:?}", result);
    }

    #[rstest]
    #[case(Weight(1.), "1".to_string())]
    #[case(Weight(1.6), "1.6".to_string())]
    fn test_display(
        #[case] input: Weight,
        #[case] expected: String
    ) {
        let result: String = format!("{}", input);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(Weight(1.), Weight(2.), Ordering::Less)]
    #[case(Weight(2.), Weight(1.), Ordering::Greater)]
    #[case(Weight(1.), Weight(1.), Ordering::Equal)]
    fn test_ord(
        #[case] weight1: Weight,
        #[case] weight2: Weight,
        #[case] expected: Ordering,
    ) {
        assert_eq!(expected, weight1.cmp(&weight2), "{}", Comparison::new(&weight1, &weight2));
    }

    #[rstest]
    #[case(1, Weight(1.))]
    fn test_from_i64(
        #[case] input: i64,
        #[case] expected: Weight,
    ) {
        let result: Weight = Weight::from(input);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(1, Weight(1.))]
    fn test_from_u64(
        #[case] input: u64,
        #[case] expected: Weight,
    ) {
        let result: Weight = Weight::from(input);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(1., Weight(1.))]
    #[case(f64::INFINITY, Weight(0.))]
    fn test_from_f64(
        #[case] input: f64,
        #[case] expected: Weight,
    ) {
        let result: Weight = Weight::from(input);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(Weight(1.), 1.)]
    fn test_to_f64(
        #[case] input: Weight,
        #[case] expected: f64,
    ) {
        let result: f64 = f64::from(input);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case("", Weight(0.))]
    #[case("1", Weight(1.))]
    fn test_from_str(
        #[case] input: &str,
        #[case] expected: Weight,
    ) {
        let result: Result<Weight> = Weight::from_str(input);

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_is_zero() {
        let input: Weight = Weight(0.);

        let result: bool = input.is_zero();

        assert!(result);
    }
}
