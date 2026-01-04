use sea_orm_migration::prelude::async_trait::async_trait;
use sea_orm_migration::prelude::{MigrationTrait, MigratorTrait};

mod m20251113_195907_create_tables;
mod m20260104_010314_add_meet_date;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251113_195907_create_tables::Migration),
            Box::new(m20260104_010314_add_meet_date::Migration)
        ]
    }
}
