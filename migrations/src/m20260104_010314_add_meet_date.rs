use sea_orm_migration::prelude::async_trait::async_trait;
use sea_orm_migration::prelude::sea_orm::{DeriveIden, DeriveMigrationName};
use sea_orm_migration::prelude::{ColumnDef, DbErr, MigrationTrait, SchemaManager, Table};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Meets::Meets)
                    .add_column(ColumnDef::new(Meets::Date).date().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Meets::Meets)
                    .drop_column(Meets::Date)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Meets {
    Meets,

    Date,
}
