use crate::models::SeaFederation;

pub enum Federation {
    FFForce,
    EPF,
    IPF,
    FFHMFAC,
    OTHER,
}

impl From<SeaFederation> for Federation {
    fn from(value: SeaFederation) -> Self {
        match value {
            SeaFederation::Ffforce => Self::FFForce,
            SeaFederation::Epf => Self::EPF,
            SeaFederation::Ipf => Self::IPF,
            SeaFederation::Ffhmfac => Self::FFHMFAC,
            SeaFederation::Other => Self::OTHER,
        }
    }
}

impl Into<SeaFederation> for Federation {
    fn into(self) -> SeaFederation {
        match self {
            Self::FFForce => SeaFederation::Ffforce,
            Self::EPF => SeaFederation::Epf,
            Self::IPF => SeaFederation::Ipf,
            Self::FFHMFAC => SeaFederation::Ffhmfac,
            Self::OTHER => SeaFederation::Other,
        }
    }
}

impl From<types::Federation> for Federation {
    fn from(value: types::Federation) -> Self {
        match value {
            types::Federation::FFForce => Self::FFForce,
            types::Federation::EPF => Self::EPF,
            types::Federation::IPF => Self::IPF,
            types::Federation::FFHMFAC => Self::FFHMFAC,
            types::Federation::OTHER => Self::OTHER,
        }
    }
}
