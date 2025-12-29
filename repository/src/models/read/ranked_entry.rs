use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};

use types::prelude::EntryDto;

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

impl Into<EntryDto> for Model {
    fn into(self) -> EntryDto {
        EntryDto {
            rank: self.rank.into(),
            name: self.name.into(),
            division: self.division.into(),
            equipment: self.equipment.into(),
            sex: self.sex.into(),
            bodyweight: self.bodyweight.into(),
            weight_class: self.weight_class.map(Decimal::into),
            squat1: self.squat1.map(Decimal::into),
            squat2: self.squat2.map(Decimal::into),
            squat3: self.squat3.map(Decimal::into),
            squat4: self.squat4.map(Decimal::into),
            bench1: self.bench1.map(Decimal::into),
            bench2: self.bench2.map(Decimal::into),
            bench3: self.bench3.map(Decimal::into),
            bench4: self.bench4.map(Decimal::into),
            deadlift1: self.deadlift1.map(Decimal::into),
            deadlift2: self.deadlift2.map(Decimal::into),
            deadlift3: self.deadlift3.map(Decimal::into),
            deadlift4: self.deadlift4.map(Decimal::into),
            best_squat: self.best_squat.map(Decimal::into),
            best_bench: self.best_bench.map(Decimal::into),
            best_deadlift: self.best_deadlift.map(Decimal::into),
            total: self.total.map(Decimal::into),
        }
    }
}
