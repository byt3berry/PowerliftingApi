use std::{fmt, num, str::FromStr};

use serde::{de::{self, Visitor}, Deserialize, Deserializer};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Weight(pub f32);

// Simple format conversions.
impl From<Weight> for f32 {
    #[inline]
    fn from(w: Weight) -> f32 {
        w.0
    }
}

impl From<Weight> for f64 {
    #[inline]
    fn from(w: Weight) -> f64 {
        w.0 as f64
    }
}

impl FromStr for Weight {
    type Err = num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Weight(0.))
        } else {
            Ok(Weight::from_f32(s.parse::<f32>()?))
        }
    }
}

struct WeightVisitor;

impl Visitor<'_> for WeightVisitor {
    type Value = Weight;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a number or numeric string")
    }

    fn visit_i64<E: de::Error>(self, i: i64) -> Result<Weight, E> {
        let v = i32::try_from(i).map_err(E::custom)?;
        Ok(Weight::from_i32(v))
    }

    fn visit_u64<E: de::Error>(self, u: u64) -> Result<Weight, E> {
        let v = i32::try_from(u).map_err(E::custom)?;
        Ok(Weight::from_i32(v))
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Weight, E> {
        Ok(Weight::from_f64(v))
    }

    fn visit_borrowed_str<E: de::Error>(self, v: &str) -> Result<Weight, E> {
        Weight::from_str(v).map_err(E::custom)
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Weight, E> {
        self.visit_borrowed_str(v)
    }
}

impl<'de> Deserialize<'de> for Weight {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Weight, D::Error> {
        deserializer.deserialize_any(WeightVisitor)
    }
}

impl Weight {
    pub fn from_f32(f: f32) -> Weight {
        if f.is_finite() {
            Weight(f)
        } else {
            Weight(0.)
        }
    }

    pub const fn from_i32(i: i32) -> Weight {
        Weight(i as f32)
    }

    pub fn from_f64(f: f64) -> Weight {
        if f.is_finite() {
            Weight(f as f32)
        } else {
            Weight(0.)
        }
    }

    pub fn is_zero(self) -> bool {
        self == Weight::from_f32(0.)
    }
}
