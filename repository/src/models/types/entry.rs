use sea_orm::ActiveValue::{NotSet, Set};
use types::EntryDto;

use crate::models::{SeaActiveEntry, SeaEntry};
use crate::models::types::division::Division;
use crate::models::types::equipment::Equipment;
use crate::models::types::sex::Sex;
use crate::models::types::weight::Weight;
use crate::models::types::weight_class::WeightClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry {
    pub name: String,
    pub division: Division,
    pub equipment: Equipment,
    pub sex: Sex,
    pub bodyweight: Weight,
    pub weight_class: WeightClass,
    pub squat1: Weight,
    pub squat2: Weight,
    pub squat3: Weight,
    pub squat4: Weight,
    pub bench1: Weight,
    pub bench2: Weight,
    pub bench3: Weight,
    pub bench4: Weight,
    pub deadlift1: Weight,
    pub deadlift2: Weight,
    pub deadlift3: Weight,
    pub deadlift4: Weight,
    pub best_squat: Weight,
    pub best_bench: Weight,
    pub best_deadlift: Weight,
    pub total: Weight,
}

impl From<EntryDto> for Entry {
    fn from(entry: EntryDto) -> Self {
        Self {
            name: entry.name.name,
            division : entry.division.into(),
            equipment : entry.equipment.into(),
            sex : entry.sex.into(),
            bodyweight : entry.bodyweight.into(),
            weight_class : entry.weight_class.into(),
            squat1 : entry.squat1.into(),
            squat2 : entry.squat2.into(),
            squat3 : entry.squat3.into(),
            squat4 : entry.squat4.into(),
            bench1 : entry.bench1.into(),
            bench2 : entry.bench2.into(),
            bench3 : entry.bench3.into(),
            bench4 : entry.bench4.into(),
            deadlift1 : entry.deadlift1.into(),
            deadlift2 : entry.deadlift2.into(),
            deadlift3 : entry.deadlift3.into(),
            deadlift4 : entry.deadlift4.into(),
            best_squat : entry.best_squat.into(),
            best_bench : entry.best_bench.into(),
            best_deadlift : entry.best_deadlift.into(),
            total : entry.total.into(),
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
            weight_class: value.weight_class.into(),
            squat1: value.squat1.into(),
            squat2: value.squat2.into(),
            squat3: value.squat3.into(),
            squat4: value.squat4.into(),
            bench1: value.bench1.into(),
            bench2: value.bench2.into(),
            bench3: value.bench3.into(),
            bench4: value.bench4.into(),
            deadlift1: value.deadlift1.into(),
            deadlift2: value.deadlift2.into(),
            deadlift3: value.deadlift3.into(),
            deadlift4: value.deadlift4.into(),
            best_squat: value.best_squat.into(),
            best_bench: value.best_bench.into(),
            best_deadlift: value.best_deadlift.into(),
            total: value.total.into(),
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
            weight_class: Set(value.weight_class.into()),
            squat1: Set(value.squat1.into()),
            squat2: Set(value.squat2.into()),
            squat3: Set(value.squat3.into()),
            squat4: Set(value.squat4.into()),
            bench1: Set(value.bench1.into()),
            bench2: Set(value.bench2.into()),
            bench3: Set(value.bench3.into()),
            bench4: Set(value.bench4.into()),
            deadlift1: Set(value.deadlift1.into()),
            deadlift2: Set(value.deadlift2.into()),
            deadlift3: Set(value.deadlift3.into()),
            deadlift4: Set(value.deadlift4.into()),
            best_squat: Set(value.best_squat.into()),
            best_bench: Set(value.best_bench.into()),
            best_deadlift: Set(value.best_deadlift.into()),
            total: Set(value.total.into()),
        }
    }
}
