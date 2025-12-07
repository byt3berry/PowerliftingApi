use anyhow::Result;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, Zero};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::str::FromStr;
use types::prelude::WeightDto;

const SCALE: u32 = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weight(pub Decimal);

impl Weight {
    #[must_use]
    pub fn is_zero(self) -> bool {
        self == Self::from(0.)
    }

    pub fn zero() -> Self {
        Self::from(Decimal::zero())
    }
}

impl From<f32> for Weight {
    fn from(value: f32) -> Self {
        match Decimal::from_f32(value) {
            Some(value) => Self(value.trunc_with_scale(SCALE)),
            None => Self::zero(),
        }
    }
}

impl From<Decimal> for Weight {
    fn from(value: Decimal) -> Self {
        Self(value)
    }
}

impl From<Weight> for WeightDto {
    fn from(value: Weight) -> Self {
        Self(value.0)
    }
}

impl From<WeightDto> for Weight {
    fn from(value: WeightDto) -> Self {
        Self(value.0)
    }
}

impl From<Weight> for Decimal {
    fn from(value: Weight) -> Self {
        value.0
    }
}

impl From<i32> for Weight {
    #[inline]
    fn from(i: i32) -> Self {
        Self(i.into())
    }
}

impl From<i64> for Weight {
    #[inline]
    fn from(i: i64) -> Self {
        Self(i.into())
    }
}

impl From<u64> for Weight {
    #[inline]
    fn from(u: u64) -> Self {
        Self(u.into())
    }
}

impl From<f64> for Weight {
    #[inline]
    fn from(value: f64) -> Self {
        match Decimal::from_f64(value) {
            Some(value) => Self(value.trunc_with_scale(SCALE)),
            None => Self::zero(),
        }
    }
}

impl FromStr for Weight {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Self::zero())
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
