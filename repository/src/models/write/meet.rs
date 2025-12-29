use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};
use types::prelude::MeetDataDto;

use crate::models::types::{Country, Federation};
use crate::models::write::entry;

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "meets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub federation: Federation,
    pub country: Country,
    pub state: String,
    pub town: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "entry::Entity")]
    Entries,
}

impl Related<entry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Entries.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<MeetDataDto> for ActiveModel {
    fn from(value: MeetDataDto) -> Self {
        Self {
            name: Set(value.name),
            federation: Set(value.federation.into()),
            country: Set(value.country.into()),
            state: Set(value.state),
            town: Set(value.town),
            ..Default::default()
        }
    }
}
