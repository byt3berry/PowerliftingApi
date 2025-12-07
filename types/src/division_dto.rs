use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum DivisionDto {
    Any,
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
