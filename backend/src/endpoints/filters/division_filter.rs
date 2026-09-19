use serde::Deserialize;
use strum_macros::Display;
use types::filters::DivisionFilterDto;
use utoipa::ToSchema;

#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, PartialEq, ToSchema)]
pub enum DivisionFilter {
    #[serde(alias = "any")]
    Any,

    #[serde(alias = "open")]
    Open,

    #[serde(alias = "g")]
    G,

    #[serde(alias = "cadet")]
    Cadet,

    #[serde(alias = "elite")]
    Elite,

    #[serde(alias = "subjuniors")]
    SubJuniors,

    #[serde(alias = "juniors")]
    Juniors,

    #[serde(alias = "masters")]
    Masters,

    #[serde(alias = "seniors")]
    Seniors,

    #[serde(alias = "masters1")]
    Masters1,

    #[serde(alias = "masters2")]
    Masters2,

    #[serde(alias = "masters3")]
    Masters3,

    #[serde(alias = "masters4")]
    Masters4,
}

impl From<DivisionFilter> for DivisionFilterDto {
    fn from(value: DivisionFilter) -> Self {
        match value {
            DivisionFilter::Any => Self::Any,
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
