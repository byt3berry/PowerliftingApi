use rust_decimal::Decimal;
use sea_orm::entity::ActiveValue;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::{ActiveModelBehavior, DeriveEntityModel};

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
            id: ActiveValue::NotSet,
            meet_id: ActiveValue::NotSet,
            name: ActiveValue::Set(value.name.into()),
            division: ActiveValue::Set(value.division.into()),
            equipment: ActiveValue::Set(value.equipment.into()),
            sex: ActiveValue::Set(value.sex.into()),
            bodyweight: ActiveValue::Set(value.bodyweight.into()),
            weight_class: ActiveValue::Set(value.weight_class.map(WeightClassDto::into)),
            squat1: ActiveValue::Set(value.squat1.map(WeightDto::into)),
            squat2: ActiveValue::Set(value.squat2.map(WeightDto::into)),
            squat3: ActiveValue::Set(value.squat3.map(WeightDto::into)),
            squat4: ActiveValue::Set(value.squat4.map(WeightDto::into)),
            bench1: ActiveValue::Set(value.bench1.map(WeightDto::into)),
            bench2: ActiveValue::Set(value.bench2.map(WeightDto::into)),
            bench3: ActiveValue::Set(value.bench3.map(WeightDto::into)),
            bench4: ActiveValue::Set(value.bench4.map(WeightDto::into)),
            deadlift1: ActiveValue::Set(value.deadlift1.map(WeightDto::into)),
            deadlift2: ActiveValue::Set(value.deadlift2.map(WeightDto::into)),
            deadlift3: ActiveValue::Set(value.deadlift3.map(WeightDto::into)),
            deadlift4: ActiveValue::Set(value.deadlift4.map(WeightDto::into)),
            best_squat: ActiveValue::Set(value.best_squat.map(WeightDto::into)),
            best_bench: ActiveValue::Set(value.best_bench.map(WeightDto::into)),
            best_deadlift: ActiveValue::Set(value.best_deadlift.map(WeightDto::into)),
            total: ActiveValue::Set(value.total.map(WeightDto::into)),
        }
    }
}
