use rust_decimal::Decimal;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};

use types::prelude::{EntryDto, WeightClassDto, WeightDto};

use crate::models::types::{Division, Equipment, Sex};
use crate::models::write::meet;

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "entries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
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

impl From<EntryDto> for ActiveModel {
    fn from(value: EntryDto) -> Self {
        Self {
            id: NotSet,
            meet_id: NotSet,
            name: Set(value.name.into()),
            division: Set(value.division.into()),
            equipment: Set(value.equipment.into()),
            sex: Set(value.sex.into()),
            bodyweight: Set(value.bodyweight.into()),
            weight_class: Set(value.weight_class.map(WeightClassDto::into)),
            squat1: Set(value.squat1.map(WeightDto::into)),
            squat2: Set(value.squat2.map(WeightDto::into)),
            squat3: Set(value.squat3.map(WeightDto::into)),
            squat4: Set(value.squat4.map(WeightDto::into)),
            bench1: Set(value.bench1.map(WeightDto::into)),
            bench2: Set(value.bench2.map(WeightDto::into)),
            bench3: Set(value.bench3.map(WeightDto::into)),
            bench4: Set(value.bench4.map(WeightDto::into)),
            deadlift1: Set(value.deadlift1.map(WeightDto::into)),
            deadlift2: Set(value.deadlift2.map(WeightDto::into)),
            deadlift3: Set(value.deadlift3.map(WeightDto::into)),
            deadlift4: Set(value.deadlift4.map(WeightDto::into)),
            best_squat: Set(value.best_squat.map(WeightDto::into)),
            best_bench: Set(value.best_bench.map(WeightDto::into)),
            best_deadlift: Set(value.best_deadlift.map(WeightDto::into)),
            total: Set(value.total.map(WeightDto::into)),
        }
    }
}
