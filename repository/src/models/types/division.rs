use sea_orm::prelude::{DeriveActiveEnum, EnumIter};
use types::prelude::DivisionDto;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "division")]
pub enum Division {
    #[sea_orm(string_value = "open")]
    Open,

    #[sea_orm(string_value = "g")]
    G,

    #[sea_orm(string_value = "cadet")]
    Cadet,

    #[sea_orm(string_value = "elite")]
    Elite,

    #[sea_orm(string_value = "subjuniors")]
    Subjuniors,

    #[sea_orm(string_value = "juniors")]
    Juniors,

    #[sea_orm(string_value = "seniors")]
    Seniors,

    #[sea_orm(string_value = "masters")]
    Masters,

    #[sea_orm(string_value = "masters1")]
    Masters1,

    #[sea_orm(string_value = "masters2")]
    Masters2,

    #[sea_orm(string_value = "masters3")]
    Masters3,

    #[sea_orm(string_value = "masters4")]
    Masters4,
}

impl From<DivisionDto> for Division {
    fn from(value: DivisionDto) -> Self {
        match value {
            DivisionDto::Open => Self::Open,
            DivisionDto::G => Self::G,
            DivisionDto::Cadet => Self::Cadet,
            DivisionDto::Elite => Self::Elite,
            DivisionDto::SubJuniors => Self::Subjuniors,
            DivisionDto::Juniors => Self::Juniors,
            DivisionDto::Masters => Self::Masters,
            DivisionDto::Seniors => Self::Seniors,
            DivisionDto::Masters1 => Self::Masters1,
            DivisionDto::Masters2 => Self::Masters2,
            DivisionDto::Masters3 => Self::Masters3,
            DivisionDto::Masters4 => Self::Masters4,
        }
    }
}

impl From<Division> for DivisionDto {
    fn from(value: Division) -> Self {
        match value {
            Division::Open => Self::Open,
            Division::G => Self::G,
            Division::Cadet => Self::Cadet,
            Division::Elite => Self::Elite,
            Division::Subjuniors => Self::SubJuniors,
            Division::Juniors => Self::Juniors,
            Division::Masters => Self::Masters,
            Division::Seniors => Self::Seniors,
            Division::Masters1 => Self::Masters1,
            Division::Masters2 => Self::Masters2,
            Division::Masters3 => Self::Masters3,
            Division::Masters4 => Self::Masters4,
        }
    }
}
