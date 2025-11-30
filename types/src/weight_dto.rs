use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeightDto(pub f32);

impl From<i64> for WeightDto {
    #[inline]
    fn from(i: i64) -> Self {
        Self(i as f32)
    }
}

impl From<u64> for WeightDto {
    #[inline]
    fn from(u: u64) -> Self {
        Self(u as f32)
    }
}

impl From<f64> for WeightDto {
    #[inline]
    fn from(f: f64) -> Self {
        if f.is_finite() {
            Self(f as f32)
        } else {
            Self(0.)
        }
    }
}

impl From<WeightDto> for f64 {
    #[inline]
    fn from(w: WeightDto) -> Self {
        Self::from(w.0)
    }
}

impl FromStr for WeightDto {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Self(0.))
        } else {
            Ok(Self::from(s.parse::<f64>()?))
        }
    }
}

impl Eq for WeightDto { }

impl Ord for WeightDto {
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

impl PartialOrd for WeightDto {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl WeightDto {
    #[must_use]
    pub fn is_zero(self) -> bool {
        self == Self::from(0.)
    }
}
