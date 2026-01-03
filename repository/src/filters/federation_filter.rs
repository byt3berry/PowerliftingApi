use migrations::SimpleExpr;
use sea_orm::{ColumnTrait, DeriveDisplay, EnumIter};

use types::filters::FederationFilterDto;

use crate::{traits::match_filter::MatchFilter};

#[derive(Clone, Copy, Debug, DeriveDisplay, EnumIter, Eq, PartialEq)]
pub enum FederationFilter {
    Any,
    AllFrench,
    FFForce,
    EPF,
    IPF,
    FFHMFAC,
}

impl FederationFilter {
    const fn can_expand(self) -> bool {
        match self {
            Self::IPF
                | Self::EPF
                | Self::AllFrench => true,
            _ => false,
        }
    }

    fn expand(self) -> Vec<FederationFilter> {
        match self {
            Self::IPF => vec![Self::IPF]
                .into_iter()
                .chain(Self::AllFrench.expand())
                .collect(),
            Self::EPF => vec![Self::EPF]
                .into_iter()
                .chain(Self::AllFrench.expand())
                .collect(),
            Self::AllFrench => vec![
                Self::FFForce,
                Self::FFHMFAC,
            ],
            Self::Any
                | Self::FFForce
                | Self::FFHMFAC => vec![self.clone()],
        }
    }
}

impl From<FederationFilterDto> for FederationFilter {
    fn from(value: FederationFilterDto) -> Self {
        match value {
            FederationFilterDto::Any => Self::Any,
            FederationFilterDto::AllFrench => Self::AllFrench,
            FederationFilterDto::FFForce => Self::FFForce,
            FederationFilterDto::EPF => Self::EPF,
            FederationFilterDto::IPF => Self::IPF,
            FederationFilterDto::FFHMFAC => Self::FFHMFAC,
        }
    }
}

impl MatchFilter for FederationFilter {
    fn eq<T>(self, column: T) -> Option<SimpleExpr>
        where T: ColumnTrait
    {
        if self == Self::Any {
            return None;
        }

        if self.can_expand() {
            let federations: Vec<String> = self
                .expand()
                .iter()
                .map(|x| x.to_string().to_lowercase())
                .collect();
            return Some(column.is_in(federations));
        }

        Some(column.eq(self.to_string().to_lowercase()))
    }
}
