use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum FederationDto {
    FFForce,
    EPF,
    IPF,
    FFHMFAC,
    OTHER,
}
