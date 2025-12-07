use serde::Deserialize;
use types::prelude::EntryDto;

use crate::types::{Division, Equipment, Sex, Username, Weight, WeightClass};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Entry {
    #[serde(rename(deserialize = "Name"))]
    pub name: Username,

    #[serde(rename(deserialize = "Division"))]
    pub division: Division,

    #[serde(rename(deserialize = "Equipment"))]
    pub equipment: Equipment,

    #[serde(rename(deserialize = "Sex"))]
    pub sex: Sex,

    #[serde(rename(deserialize = "BodyweightKg"))]
    pub bodyweight: Weight,

    #[serde(rename(deserialize = "WeightClassKg"))]
    pub weight_class: Option<WeightClass>,

    #[serde(rename(deserialize = "Squat1Kg"))]
    pub squat1: Option<Weight>,

    #[serde(rename(deserialize = "Squat2Kg"))]
    pub squat2: Option<Weight>,

    #[serde(rename(deserialize = "Squat3Kg"))]
    pub squat3: Option<Weight>,

    #[serde(rename(deserialize = "Best3SquatKg"))]
    pub best3squat: Option<Weight>,

    #[serde(rename(deserialize = "Bench1Kg"))]
    pub bench1: Option<Weight>,

    #[serde(rename(deserialize = "Bench2Kg"))]
    pub bench2: Option<Weight>,

    #[serde(rename(deserialize = "Bench3Kg"))]
    pub bench3: Option<Weight>,

    #[serde(rename(deserialize = "Best3BenchKg"))]
    pub best3bench: Option<Weight>,

    #[serde(rename(deserialize = "Deadlift1Kg"))]
    pub deadlift1: Option<Weight>,

    #[serde(rename(deserialize = "Deadlift2Kg"))]
    pub deadlift2: Option<Weight>,

    #[serde(rename(deserialize = "Deadlift3Kg"))]
    pub deadlift3: Option<Weight>,

    #[serde(rename(deserialize = "Best3DeadliftKg"))]
    pub best3deadlift: Option<Weight>,

    #[serde(rename(deserialize = "TotalKg"))]
    pub total: Option<Weight>,
}

impl From<Entry> for EntryDto {
    fn from(value: Entry) -> Self {
        Self {
            name: value.name.into(),
            division: value.division.into(),
            equipment: value.equipment.into(),
            sex: value.sex.into(),
            bodyweight: value.bodyweight.into(),
            weight_class: value.weight_class.map(WeightClass::into),
            squat1: value.squat1.map(Weight::into),
            squat2: value.squat2.map(Weight::into),
            squat3: value.squat3.map(Weight::into),
            squat4: None,
            bench1: value.bench1.map(Weight::into),
            bench2: value.bench2.map(Weight::into),
            bench3: value.bench3.map(Weight::into),
            bench4: None,
            deadlift1: value.deadlift1.map(Weight::into),
            deadlift2: value.deadlift2.map(Weight::into),
            deadlift3: value.deadlift3.map(Weight::into),
            deadlift4: None,
            best_squat: value.best3squat.map(Weight::into),
            best_bench: value.best3bench.map(Weight::into),
            best_deadlift: value.best3deadlift.map(Weight::into),
            total: value.total.map(Weight::into),
        }
    }
}
