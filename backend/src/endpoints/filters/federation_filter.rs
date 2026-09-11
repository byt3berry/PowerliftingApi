use serde::Deserialize;
use strum_macros::Display;
use types::filters::FederationFilterDto;
use utoipa::ToSchema;

#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, PartialEq, ToSchema)]
pub enum FederationFilter {
    FFForce,
    EPF,
    IPF,
    FFHMFAC,
    ANY,
}

impl From<FederationFilter> for FederationFilterDto {
    fn from(value: FederationFilter) -> Self {
        match value {
            FederationFilter::FFForce => Self::FFForce,
            FederationFilter::EPF => Self::EPF,
            FederationFilter::IPF => Self::IPF,
            FederationFilter::FFHMFAC => Self::FFHMFAC,
            FederationFilter::ANY => Self::Any,
        }
    }
}
