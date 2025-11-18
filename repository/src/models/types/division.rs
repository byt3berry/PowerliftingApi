use crate::models::SeaDivision;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Division {
    Any,
    Open,
    G,
    Cadet,
    Elite,
    SubJuniors,
    Juniors,
    Seniors,
    Masters,
    Masters1,
    Masters2,
    Masters3,
    Masters4
}

impl From<types::Division> for Division {
    fn from(value: types::Division) -> Self {
        match value {
            types::Division::Any => Self::Any,
            types::Division::Open => Self::Open,
            types::Division::G => Self::G,
            types::Division::Cadet => Self::Cadet,
            types::Division::Elite => Self::Elite,
            types::Division::SubJuniors => Self::SubJuniors,
            types::Division::Juniors => Self::Juniors,
            types::Division::Masters => Self::Masters,
            types::Division::Seniors => Self::Seniors,
            types::Division::Masters1 => Self::Masters1,
            types::Division::Masters2 => Self::Masters2,
            types::Division::Masters3 => Self::Masters3,
            types::Division::Masters4 => Self::Masters4,
        }
    }
}

impl From<SeaDivision> for Division {
    fn from(value: SeaDivision) -> Self {
        match value {
            SeaDivision::Any => Self::Any,
            SeaDivision::Open => Self::Open,
            SeaDivision::G => Self::G,
            SeaDivision::Cadet => Self::Cadet,
            SeaDivision::Elite => Self::Elite,
            SeaDivision::Subjuniors => Self::SubJuniors,
            SeaDivision::Juniors => Self::Juniors,
            SeaDivision::Masters => Self::Masters,
            SeaDivision::Seniors => Self::Seniors,
            SeaDivision::Masters1 => Self::Masters1,
            SeaDivision::Masters2 => Self::Masters2,
            SeaDivision::Masters3 => Self::Masters3,
            SeaDivision::Masters4 => Self::Masters4,
        }
    }
}

impl Into<SeaDivision> for Division {
    fn into(self) -> SeaDivision {
        match self {
            Self::Any => SeaDivision::Any,
            Self::Open => SeaDivision::Open,
            Self::G => SeaDivision::G,
            Self::Cadet => SeaDivision::Cadet,
            Self::Elite => SeaDivision::Elite,
            Self::SubJuniors => SeaDivision::Subjuniors,
            Self::Juniors => SeaDivision::Juniors,
            Self::Masters => SeaDivision::Masters,
            Self::Seniors => SeaDivision::Seniors,
            Self::Masters1 => SeaDivision::Masters1,
            Self::Masters2 => SeaDivision::Masters2,
            Self::Masters3 => SeaDivision::Masters3,
            Self::Masters4 => SeaDivision::Masters4,
        }
    }
}
