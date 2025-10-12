use diesel::pg::Pg;
use diesel::{Queryable, Selectable};

use crate::models::types::division::Division;
use crate::models::types::equipment::Equipment;
use crate::models::types::sex::Sex;
use crate::models::types::weight::Weight;
use crate::models::types::weight_class::WeightClass;
use crate::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::entries)]
#[diesel(belongs_to(Meet))]
#[diesel(check_for_backend(Pg))]
pub struct Entry {
    pub id: i32,
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
