use diesel::Insertable;
use types::EntryDto;

use crate::models::types::division::Division;
use crate::models::types::equipment::Equipment;
use crate::models::types::sex::Sex;
use crate::models::types::weight::Weight;
use crate::models::types::weight_class::WeightClass;
use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::entries)]
pub struct NewEntry {
    pub meet_id: i32,
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

impl NewEntry {
    pub fn from(meet_id: i32, entry: &EntryDto) -> Self {
        Self {
            meet_id,
            name: entry.name.name.clone(),
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
