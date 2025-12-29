use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum DivisionDto {
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
