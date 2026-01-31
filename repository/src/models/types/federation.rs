use sea_orm::prelude::{DeriveActiveEnum, DeriveDisplay, EnumIter};
use types::prelude::FederationDto;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveDisplay, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "federation")]
pub enum Federation {
    #[sea_orm(string_value = "ffforce")]
    Ffforce,

    #[sea_orm(string_value = "epf")]
    Epf,

    #[sea_orm(string_value = "ipf")]
    Ipf,

    #[sea_orm(string_value = "ffhmfac")]
    Ffhmfac,

    #[sea_orm(string_value = "other")]
    Other,
}

impl From<FederationDto> for Federation {
    fn from(value: FederationDto) -> Self {
        match value {
            FederationDto::FFForce => Self::Ffforce,
            FederationDto::EPF => Self::Epf,
            FederationDto::IPF => Self::Ipf,
            FederationDto::FFHMFAC => Self::Ffhmfac,
            FederationDto::OTHER => Self::Other,
        }
    }
}

impl From<Federation> for FederationDto {
    fn from(value: Federation) -> Self {
        match value {
            Federation::Ffforce => Self::FFForce,
            Federation::Epf => Self::EPF,
            Federation::Ipf => Self::IPF,
            Federation::Ffhmfac => Self::FFHMFAC,
            Federation::Other => Self::OTHER,
        }
    }
}
