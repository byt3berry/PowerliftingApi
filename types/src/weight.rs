use anyhow::Result;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::cmp::Ordering;
use std::fmt::Display;
use std::str::FromStr;
use std::{fmt, num};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Weight(pub f32);

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
    type Err = num::ParseFloatError;

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

    fn visit_i64<E: Error>(self, i: i64) -> Result<Self::Value, E> {
        Ok(Self::Value::from(i))
    }

    fn visit_u64<E: Error>(self, u: u64) -> Result<Self::Value, E> {
        Ok(Self::Value::from(u))
    }

    fn visit_f64<E: Error>(self, f: f64) -> Result<Self::Value, E> {
        Ok(Self::Value::from(f))
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Self::Value::from_str(v).map_err(E::custom)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::Comparison;
    use rstest::rstest;
    use std::cmp::Ordering;

    use super::Weight;

    #[rstest]
    #[case(Weight::from(1.), Weight::from(2.), Ordering::Less)]
    #[case(Weight::from(2.), Weight::from(1.), Ordering::Greater)]
    #[case(Weight::from(1.), Weight::from(1.), Ordering::Equal)]
    fn test_ord(
        #[case] weight1: Weight,
        #[case] weight2: Weight,
        #[case] expected: Ordering,
    ) {
        assert_eq!(weight1.cmp(&weight2), expected, "{}", Comparison::new(&weight1, &weight2));
    }
}
