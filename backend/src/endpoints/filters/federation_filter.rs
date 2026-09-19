use serde::Deserialize;
use strum_macros::Display;
use types::filters::FederationFilterDto;
use utoipa::ToSchema;

#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, PartialEq, ToSchema)]
pub enum FederationFilter {
    #[serde(alias = "any")]
    Any,

    #[serde(alias = "ffforce")]
    Ffforce,

    #[serde(alias = "epf")]
    Epf,

    #[serde(alias = "ipf")]
    Ipf,

    #[serde(alias = "ffhmfac")]
    Ffhmfac,
}

impl From<FederationFilter> for FederationFilterDto {
    fn from(value: FederationFilter) -> Self {
        match value {
            FederationFilter::Any => Self::Any,
            FederationFilter::Ffforce => Self::FFForce,
            FederationFilter::Epf => Self::EPF,
            FederationFilter::Ipf => Self::IPF,
            FederationFilter::Ffhmfac => Self::FFHMFAC,
        }
    }
}
