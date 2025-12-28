use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};

use crate::models::{read::ranked_entry, SeaCountry, SeaFederation};

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "meets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub federation: SeaFederation,
    pub country: SeaCountry,
    pub state: String,
    pub town: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "ranked_entry::Entity")]
    Entries,
}

impl Related<ranked_entry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Entries.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
