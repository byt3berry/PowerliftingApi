use serde::Deserialize;
use strum_macros::Display;
use types::filters::SexFilterDto;
use utoipa::ToSchema;

#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, PartialEq, ToSchema)]
pub enum SexFilter {
    #[serde(alias = "any")]
    Any,

    #[serde(alias = "m")]
    M,

    #[serde(alias = "f")]
    F,
}

impl From<SexFilter> for SexFilterDto {
    fn from(value: SexFilter) -> Self {
        match value {
            SexFilter::Any => Self::Any,
            SexFilter::M => Self::M,
            SexFilter::F => Self::F,
        }
    }
}
