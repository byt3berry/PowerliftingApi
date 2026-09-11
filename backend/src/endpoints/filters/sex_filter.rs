use serde::Deserialize;
use strum_macros::Display;
use types::filters::SexFilterDto;
use utoipa::ToSchema;

#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, PartialEq, ToSchema)]
pub enum SexFilter {
    M,
    F,
}

impl From<SexFilter> for SexFilterDto {
    fn from(value: SexFilter) -> Self {
        match value {
            SexFilter::M => Self::M,
            SexFilter::F => Self::F,
        }
    }
}
