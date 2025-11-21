use types::FederationDto;

use crate::models::SeaFederation;

pub enum Federation {
    Ffforce,
    Epf,
    Ipf,
    Ffhmfac,
    Other,
}

impl From<FederationDto> for Federation {
    fn from(value: FederationDto) -> Self {
        match value {
            types::FederationDto::FFForce => Self::Ffforce,
            types::FederationDto::EPF => Self::Epf,
            types::FederationDto::IPF => Self::Ipf,
            types::FederationDto::FFHMFAC => Self::Ffhmfac,
            types::FederationDto::OTHER => Self::Other,
        }
    }
}

impl From<SeaFederation> for Federation {
    fn from(value: SeaFederation) -> Self {
        match value {
            SeaFederation::Ffforce => Self::Ffforce,
            SeaFederation::Epf => Self::Epf,
            SeaFederation::Ipf => Self::Ipf,
            SeaFederation::Ffhmfac => Self::Ffhmfac,
            SeaFederation::Other => Self::Other,
        }
    }
}

impl From<Federation> for SeaFederation {
    fn from(value: Federation) -> Self {
        match value {
            Federation::Ffforce => Self::Ffforce,
            Federation::Epf => Self::Epf,
            Federation::Ipf => Self::Ipf,
            Federation::Ffhmfac => Self::Ffhmfac,
            Federation::Other => Self::Other,
        }
    }
}
