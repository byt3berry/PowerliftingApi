use rust_decimal::Decimal;
use sea_orm::FromQueryResult;

use types::prelude::EntryDto;

use crate::models::types::{Division, Equipment, Sex, Username, Weight, WeightClass};

#[derive(Clone, Debug, Eq, PartialEq, FromQueryResult)]
pub struct RankedEntry {
    pub rank: i64,
    pub name: Username,
    pub division: Division,
    pub equipment: Equipment,
    pub sex: Sex,
    pub bodyweight: Decimal,
    pub weight_class: Option<WeightClass>,
    pub squat1: Option<Weight>,
    pub squat2: Option<Weight>,
    pub squat3: Option<Weight>,
    pub squat4: Option<Weight>,
    pub bench1: Option<Weight>,
    pub bench2: Option<Weight>,
    pub bench3: Option<Weight>,
    pub bench4: Option<Weight>,
    pub deadlift1: Option<Weight>,
    pub deadlift2: Option<Weight>,
    pub deadlift3: Option<Weight>,
    pub deadlift4: Option<Weight>,
    pub best_squat: Option<Weight>,
    pub best_bench: Option<Weight>,
    pub best_deadlift: Option<Weight>,
    pub total: Option<Weight>,
}

impl From<RankedEntry> for EntryDto {
    fn from(value: RankedEntry) -> Self {
        Self {
            rank: value.rank.into(),
            name: value.name.into(),
            division: value.division.into(),
            equipment: value.equipment.into(),
            sex: value.sex.into(),
            bodyweight: value.bodyweight.into(),
            weight_class: value.weight_class.map(WeightClass::into),
            squat1: value.squat1.map(Weight::into),
            squat2: value.squat2.map(Weight::into),
            squat3: value.squat3.map(Weight::into),
            squat4: value.squat4.map(Weight::into),
            bench1: value.bench1.map(Weight::into),
            bench2: value.bench2.map(Weight::into),
            bench3: value.bench3.map(Weight::into),
            bench4: value.bench4.map(Weight::into),
            deadlift1: value.deadlift1.map(Weight::into),
            deadlift2: value.deadlift2.map(Weight::into),
            deadlift3: value.deadlift3.map(Weight::into),
            deadlift4: value.deadlift4.map(Weight::into),
            best_squat: value.best_squat.map(Weight::into),
            best_bench: value.best_bench.map(Weight::into),
            best_deadlift: value.best_deadlift.map(Weight::into),
            total: value.total.map(Weight::into),
        }
    }
}
