use rust_decimal::Decimal;
use sea_orm::ActiveValue::{NotSet, Set};
use types::prelude::*;

use crate::models::types::{Division, Equipment, Sex, Weight, WeightClass};
use crate::models::{SeaActiveEntry, SeaEntry};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry {
    pub name: String,
    pub division: Division,
    pub equipment: Equipment,
    pub sex: Sex,
    pub bodyweight: Weight,
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

impl From<EntryDto> for Entry {
    fn from(entry: EntryDto) -> Self {
        Self {
            name: entry.name.name,
            division : entry.division.into(),
            equipment : entry.equipment.into(),
            sex : entry.sex.into(),
            bodyweight : entry.bodyweight.into(),
            weight_class : entry.weight_class.map(WeightClassDto::into),
            squat1 : entry.squat1.map(WeightDto::into),
            squat2 : entry.squat2.map(WeightDto::into),
            squat3 : entry.squat3.map(WeightDto::into),
            squat4 : entry.squat4.map(WeightDto::into),
            bench1 : entry.bench1.map(WeightDto::into),
            bench2 : entry.bench2.map(WeightDto::into),
            bench3 : entry.bench3.map(WeightDto::into),
            bench4 : entry.bench4.map(WeightDto::into),
            deadlift1 : entry.deadlift1.map(WeightDto::into),
            deadlift2 : entry.deadlift2.map(WeightDto::into),
            deadlift3 : entry.deadlift3.map(WeightDto::into),
            deadlift4 : entry.deadlift4.map(WeightDto::into),
            best_squat : entry.best_squat.map(WeightDto::into),
            best_bench : entry.best_bench.map(WeightDto::into),
            best_deadlift : entry.best_deadlift.map(WeightDto::into),
            total : entry.total.map(WeightDto::into),
        }
    }
}

impl From<SeaEntry> for Entry {
    fn from(value: SeaEntry) -> Self {
        Self {
            name: value.name,
            division: value.division.into(),
            equipment: value.equipment.into(),
            sex: value.sex.into(),
            bodyweight: value.bodyweight.into(),
            weight_class: value.weight_class.map(Decimal::into),
            squat1: value.squat1.map(Decimal::into),
            squat2: value.squat2.map(Decimal::into),
            squat3: value.squat3.map(Decimal::into),
            squat4: value.squat4.map(Decimal::into),
            bench1: value.bench1.map(Decimal::into),
            bench2: value.bench2.map(Decimal::into),
            bench3: value.bench3.map(Decimal::into),
            bench4: value.bench4.map(Decimal::into),
            deadlift1: value.deadlift1.map(Decimal::into),
            deadlift2: value.deadlift2.map(Decimal::into),
            deadlift3: value.deadlift3.map(Decimal::into),
            deadlift4: value.deadlift4.map(Decimal::into),
            best_squat: value.best_squat.map(Decimal::into),
            best_bench: value.best_bench.map(Decimal::into),
            best_deadlift: value.best_deadlift.map(Decimal::into),
            total: value.total.map(Decimal::into),
        }
    }
}

impl From<Entry> for SeaActiveEntry {
    fn from(value: Entry) -> Self {
        Self {
            id: NotSet,
            meet_id: NotSet,
            name: Set(value.name),
            division: Set(value.division.into()),
            equipment: Set(value.equipment.into()),
            sex: Set(value.sex.into()),
            bodyweight: Set(value.bodyweight.into()),
            weight_class: Set(value.weight_class.map(WeightClass::into)),
            squat1: Set(value.squat1.map(Weight::into)),
            squat2: Set(value.squat2.map(Weight::into)),
            squat3: Set(value.squat3.map(Weight::into)),
            squat4: Set(value.squat4.map(Weight::into)),
            bench1: Set(value.bench1.map(Weight::into)),
            bench2: Set(value.bench2.map(Weight::into)),
            bench3: Set(value.bench3.map(Weight::into)),
            bench4: Set(value.bench4.map(Weight::into)),
            deadlift1: Set(value.deadlift1.map(Weight::into)),
            deadlift2: Set(value.deadlift2.map(Weight::into)),
            deadlift3: Set(value.deadlift3.map(Weight::into)),
            deadlift4: Set(value.deadlift4.map(Weight::into)),
            best_squat: Set(value.best_squat.map(Weight::into)),
            best_bench: Set(value.best_bench.map(Weight::into)),
            best_deadlift: Set(value.best_deadlift.map(Weight::into)),
            total: Set(value.total.map(Weight::into)),
        }
    }
}
