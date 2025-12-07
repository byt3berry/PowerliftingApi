use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeightClassDto {
    UnderOrEqual(WeightDto),
    Over(WeightDto),
}

impl ToString for WeightClassDto {
    fn to_string(&self) -> String {
        match self {
            WeightClassDto::UnderOrEqual(weight) => weight.to_string(),
            WeightClassDto::Over(weight) => weight.to_string(),
        }
    }
}
