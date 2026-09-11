use serde::Deserialize;
use strum_macros::Display;
use types::filters::DivisionFilterDto;
use utoipa::ToSchema;

#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, PartialEq, ToSchema)]
pub enum DivisionFilter {
    Open,
    G,
    Cadet,
    Elite,
    SubJuniors,
    Juniors,
    Masters,
    Seniors,
    Masters1,
    Masters2,
    Masters3,
    Masters4,
}

impl From<DivisionFilter> for DivisionFilterDto {
    fn from(value: DivisionFilter) -> Self {
        match value {
            DivisionFilter::Open => Self::Open,
            DivisionFilter::G => Self::G,
            DivisionFilter::Cadet => Self::Cadet,
            DivisionFilter::Elite => Self::Elite,
            DivisionFilter::SubJuniors => Self::SubJuniors,
            DivisionFilter::Juniors => Self::Juniors,
            DivisionFilter::Masters => Self::Masters,
            DivisionFilter::Seniors => Self::Seniors,
            DivisionFilter::Masters1 => Self::Masters1,
            DivisionFilter::Masters2 => Self::Masters2,
            DivisionFilter::Masters3 => Self::Masters3,
            DivisionFilter::Masters4 => Self::Masters4,
        }
    }
}
