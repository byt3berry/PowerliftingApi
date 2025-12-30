use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};

use crate::models::types::{Division, Equipment, Sex};
use crate::models::read::meet;

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "entries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub rank: i64,
    pub meet_id: i32,
    pub name: String,
    pub division: Division,
    pub equipment: Equipment,
    pub sex: Sex,
    pub bodyweight: Decimal,
    pub weight_class: Option<Decimal>,
    pub squat1: Option<Decimal>,
    pub squat2: Option<Decimal>,
    pub squat3: Option<Decimal>,
    pub squat4: Option<Decimal>,
    pub bench1: Option<Decimal>,
    pub bench2: Option<Decimal>,
    pub bench3: Option<Decimal>,
    pub bench4: Option<Decimal>,
    pub deadlift1: Option<Decimal>,
    pub deadlift2: Option<Decimal>,
    pub deadlift3: Option<Decimal>,
    pub deadlift4: Option<Decimal>,
    pub best_squat: Option<Decimal>,
    pub best_bench: Option<Decimal>,
    pub best_deadlift: Option<Decimal>,
    pub total: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "meet::Entity")]
    Meets,
}

impl Related<meet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Meets.def()
    }
}

impl ActiveModelBehavior for ActiveModel { }
