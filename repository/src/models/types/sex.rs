use sea_orm::prelude::{DeriveActiveEnum, EnumIter};
use types::prelude::SexDto;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "sex")]
pub enum Sex {
    #[sea_orm(string_value = "m")]
    M,

    #[sea_orm(string_value = "f")]
    F,
}

impl From<SexDto> for Sex {
    fn from(value: SexDto) -> Self {
        match value {
            SexDto::M => Self::M,
            SexDto::F => Self::F,
        }
    }
}

impl From<Sex> for SexDto {
    fn from(value: Sex) -> Self {
        match value {
            Sex::M => Self::M,
            Sex::F => Self::F,
        }
    }
}
