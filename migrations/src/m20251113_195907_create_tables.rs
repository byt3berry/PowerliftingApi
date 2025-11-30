use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::sea_orm::{DeriveIden, DeriveMigrationName};
use sea_orm_migration::prelude::{async_trait, ColumnDef, DbErr, ForeignKey, ForeignKeyAction, MigrationTrait, SchemaManager, Table};

const PRECISION: u32 = 8;
const SCALE: u32 = 4;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Federation::Federation)
                    .values([
                        Federation::Ffforce,
                        Federation::Epf,
                        Federation::Ipf,
                        Federation::Ffhmfac,
                        Federation::Other,
                    ])
                    .to_owned()
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Country::Country)
                    .values([
                        Country::France,
                        Country::Other,
                    ])
                    .to_owned()
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Division::Division)
                    .values([
                        Division::Any,
                        Division::Open,
                        Division::G,
                        Division::Cadet,
                        Division::Elite,
                        Division::Subjuniors,
                        Division::Juniors,
                        Division::Seniors,
                        Division::Masters,
                        Division::Masters1,
                        Division::Masters2,
                        Division::Masters3,
                        Division::Masters4,
                    ])
                    .to_owned()
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Equipment::Equipment)
                    .values([
                        Equipment::Any,
                        Equipment::Raw,
                        Equipment::Wraps,
                        Equipment::Single,
                        Equipment::Multi,
                        Equipment::Straps,
                        Equipment::Sleeves,
                        Equipment::Bare,
                        Equipment::Unlimited,
                    ])
                    .to_owned()
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Sex::Sex)
                    .values([
                        Sex::M,
                        Sex::F,
                    ])
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Meets::Meets)
                    .if_not_exists()
                    .col(ColumnDef::new(Meets::Id).integer().unique_key().primary_key().auto_increment().not_null())
                    .col(ColumnDef::new(Meets::Name).string_len(256).not_null())
                    .col(ColumnDef::new(Meets::Federation).custom(Federation::Federation).not_null())
                    .col(ColumnDef::new(Meets::Country).custom(Country::Country).not_null())
                    .col(ColumnDef::new(Meets::State).string_len(256).not_null())
                    .col(ColumnDef::new(Meets::Town).string_len(256).not_null())
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                .table(Entries::Entries)
                .if_not_exists()
                .col(ColumnDef::new(Entries::Id).integer().unique_key().primary_key().auto_increment().not_null())
                .col(ColumnDef::new(Entries::Meetid).integer().not_null())
                .col(ColumnDef::new(Entries::Name).string_len(256).not_null())
                .col(ColumnDef::new(Entries::Division).custom(Division::Division).not_null())
                .col(ColumnDef::new(Entries::Equipment).custom(Equipment::Equipment).not_null())
                .col(ColumnDef::new(Entries::Sex).custom(Sex::Sex).not_null())
                .col(ColumnDef::new(Entries::Bodyweight).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::Weightclass).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Squat1).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Squat2).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Squat3).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Squat4).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Bench1).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Bench2).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Bench3).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Bench4).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Deadlift1).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Deadlift2).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Deadlift3).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Deadlift4).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Bestsquat).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Bestbench).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::BestDeadlift).decimal_len(PRECISION, SCALE))
                .col(ColumnDef::new(Entries::Total).decimal_len(PRECISION, SCALE))
                .foreign_key(
                    ForeignKey::create()
                    .from(Entries::Entries, Entries::Meetid)
                    .to(Meets::Meets, Meets::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                )
                .to_owned()
                )
                .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Entries::Entries)
                    .if_exists()
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Meets::Meets)
                    .if_exists()
                    .to_owned()
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                .name(Sex::Sex)
                .if_exists()
                .to_owned()
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                .name(Equipment::Equipment)
                .if_exists()
                .to_owned()
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(Division::Division)
                    .if_exists()
                    .to_owned()
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(Country::Country)
                    .if_exists()
                    .to_owned()
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(Federation::Federation)
                    .if_exists()
                    .to_owned()
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Federation {
    Federation,

    Ffforce,
    Epf,
    Ipf,
    Ffhmfac,
    Other,
}

#[derive(DeriveIden)]
enum Country  {
    Country,

    France,
    Other,
}

#[derive(DeriveIden)]
enum Division {
    Division,

    Any,
    Open,
    G,
    Cadet,
    Elite,
    Subjuniors,
    Juniors,
    Seniors,
    Masters,
    Masters1,
    Masters2,
    Masters3,
    Masters4
}

#[derive(DeriveIden)]
enum Equipment {
    Equipment,

    Any,
    Raw,
    Wraps,
    Single,
    Multi,
    Straps,
    Sleeves,
    Bare,
    Unlimited
}

#[derive(DeriveIden)]
enum Sex {
    Sex,

    M,
    F,
}

#[derive(DeriveIden)]
#[sea_orm(enum_name = "entries")]
enum Entries {
    Entries,

    Id,
    #[sea_orm(iden = "meet_id")]
    Meetid,
    Name,
    Division,
    Equipment,
    Sex,
    Bodyweight,
    #[sea_orm(iden = "weight_class")]
    Weightclass,
    Squat1,
    Squat2,
    Squat3,
    Squat4,
    Bench1,
    Bench2,
    Bench3,
    Bench4,
    Deadlift1,
    Deadlift2,
    Deadlift3,
    Deadlift4,
    #[sea_orm(iden = "best_squat")]
    Bestsquat,
    #[sea_orm(iden = "best_bench")]
    Bestbench,
    #[sea_orm(iden = "best_deadlift")]
    BestDeadlift,
    Total,
}

#[derive(DeriveIden)]
enum Meets {
    Meets,

    Id,
    Name,
    Federation,
    Country,
    State,
    Town,
}
