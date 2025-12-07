use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum SexDto {
    M,
    F,
}
