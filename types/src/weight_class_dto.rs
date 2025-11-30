use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeightClassDto {
    UnderOrEqual(WeightDto),
    Over(WeightDto),
    None,
}
