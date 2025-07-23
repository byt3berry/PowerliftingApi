use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::{fmt, num, str::FromStr};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

    fn visit_i64<E: de::Error>(self, i: i64) -> Result<Self::Value, E> {
        Ok(Self::Value::from(i))
    }

    fn visit_u64<E: de::Error>(self, u: u64) -> Result<Self::Value, E> {
        Ok(Self::Value::from(u))
    }

    fn visit_f64<E: de::Error>(self, f: f64) -> Result<Self::Value, E> {
        Ok(Self::Value::from(f))
    }

    fn visit_borrowed_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Self::Value::from_str(v).map_err(E::custom)
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        self.visit_borrowed_str(v)
    }
}
