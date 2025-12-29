use sea_orm::prelude::{DeriveActiveEnum, EnumIter};
use types::prelude::CountryDto;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "country")]
pub enum Country {
    #[sea_orm(string_value = "france")]
    France,

    #[sea_orm(string_value = "other")]
    Other,
}

impl From<CountryDto> for Country {
    fn from(value: CountryDto) -> Self {
        match value {
            CountryDto::FRANCE => Self::France,
            CountryDto::OTHER => Self::Other,
        }
    }
}
