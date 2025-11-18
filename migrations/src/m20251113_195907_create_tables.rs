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
                    .as_enum(Federation::FEDERATION)
                    .values([
                        Federation::FFFORCE,
                        Federation::EPF,
                        Federation::IPF,
                        Federation::FFHMFAC,
                        Federation::OTHER,
                    ])
                    .to_owned()
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Country::COUNTRY)
                    .values([
                        Country::FRANCE,
                        Country::OTHER,
                    ])
                    .to_owned()
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Division::DIVISION)
                    .values([
                        Division::ANY,
                        Division::OPEN,
                        Division::G,
                        Division::CADET,
                        Division::ELITE,
                        Division::SUBJUNIORS,
                        Division::JUNIORS,
                        Division::SENIORS,
                        Division::MASTERS,
                        Division::MASTERS1,
                        Division::MASTERS2,
                        Division::MASTERS3,
                        Division::MASTERS4,
                    ])
                    .to_owned()
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Equipment::EQUIPMENT)
                    .values([
                        Equipment::ANY,
                        Equipment::RAW,
                        Equipment::WRAPS,
                        Equipment::SINGLE,
                        Equipment::MULTI,
                        Equipment::STRAPS,
                        Equipment::SLEEVES,
                        Equipment::BARE,
                        Equipment::UNLIMITED,
                    ])
                    .to_owned()
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Sex::SEX)
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
                    .table(Meets::MEETS)
                    .if_not_exists()
                    .col(ColumnDef::new(Meets::ID).integer().unique_key().primary_key().auto_increment().not_null())
                    .col(ColumnDef::new(Meets::NAME).string_len(256).not_null())
                    .col(ColumnDef::new(Meets::FEDERATION).custom(Federation::FEDERATION).not_null())
                    .col(ColumnDef::new(Meets::COUNTRY).custom(Country::COUNTRY).not_null())
                    .col(ColumnDef::new(Meets::STATE).string_len(256).not_null())
                    .col(ColumnDef::new(Meets::TOWN).string_len(256).not_null())
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                .table(Entries::ENTRIES)
                .if_not_exists()
                .col(ColumnDef::new(Entries::ID).integer().unique_key().primary_key().auto_increment().not_null())
                .col(ColumnDef::new(Entries::NAME).string_len(256).not_null())
                .col(ColumnDef::new(Entries::DIVISION).custom(Division::DIVISION).not_null())
                .col(ColumnDef::new(Entries::EQUIPMENT).custom(Equipment::EQUIPMENT).not_null())
                .col(ColumnDef::new(Entries::SEX).custom(Sex::SEX).not_null())
                .col(ColumnDef::new(Entries::BODYWEIGHT).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::WEIGHTCLASS).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::SQUAT1).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::SQUAT2).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::SQUAT3).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::SQUAT4).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::BENCH1).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::BENCH2).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::BENCH3).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::BENCH4).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::DEADLIFT1).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::DEADLIFT2).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::DEADLIFT3).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::DEADLIFT4).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::BESTSQUAT).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::BESTBENCH).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::BESTDEADLIFT).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::TOTAL).decimal_len(PRECISION, SCALE).not_null())
                .col(ColumnDef::new(Entries::MEETID).integer().not_null())
                .foreign_key(
                    ForeignKey::create()
                    .from(Entries::ENTRIES, Entries::MEETID)
                    .to(Meets::MEETS, Meets::ID)
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
                    .table(Entries::ENTRIES)
                    .if_exists()
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Meets::MEETS)
                    .if_exists()
                    .to_owned()
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                .name(Sex::SEX)
                .if_exists()
                .to_owned()
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                .name(Equipment::EQUIPMENT)
                .if_exists()
                .to_owned()
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(Division::DIVISION)
                    .if_exists()
                    .to_owned()
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(Country::COUNTRY)
                    .if_exists()
                    .to_owned()
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(Federation::FEDERATION)
                    .if_exists()
                    .to_owned()
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Federation {
    FEDERATION,

    FFFORCE,
    EPF,
    IPF,
    FFHMFAC,
    OTHER,
}

#[derive(DeriveIden)]
enum Country  {
    COUNTRY,

    FRANCE,
    OTHER,
}

#[derive(DeriveIden)]
enum Division {
    DIVISION,

    ANY,
    OPEN,
    G,
    CADET,
    ELITE,
    SUBJUNIORS,
    JUNIORS,
    SENIORS,
    MASTERS,
    MASTERS1,
    MASTERS2,
    MASTERS3,
    MASTERS4
}

#[derive(DeriveIden)]
enum Equipment {
    EQUIPMENT,

    ANY,
    RAW,
    WRAPS,
    SINGLE,
    MULTI,
    STRAPS,
    SLEEVES,
    BARE,
    UNLIMITED
}

#[derive(DeriveIden)]
enum Sex {
    SEX,

    M,
    F,
}

#[derive(DeriveIden)]
#[sea_orm(enum_name = "entries")]
enum Entries {
    ENTRIES,

    ID,
    #[sea_orm(iden = "meet_id")]
    MEETID,
    NAME,
    DIVISION,
    EQUIPMENT,
    SEX,
    BODYWEIGHT,
    #[sea_orm(iden = "weight_class")]
    WEIGHTCLASS,
    SQUAT1,
    SQUAT2,
    SQUAT3,
    SQUAT4,
    BENCH1,
    BENCH2,
    BENCH3,
    BENCH4,
    DEADLIFT1,
    DEADLIFT2,
    DEADLIFT3,
    DEADLIFT4,
    #[sea_orm(iden = "best_squat")]
    BESTSQUAT,
    #[sea_orm(iden = "best_bench")]
    BESTBENCH,
    #[sea_orm(iden = "best_deadlift")]
    BESTDEADLIFT,
    TOTAL,
}

#[derive(DeriveIden)]
enum Meets {
    MEETS,

    ID,
    NAME,
    FEDERATION,
    COUNTRY,
    STATE,
    TOWN,
}
