use crate::{Country, Federation};

pub struct Meet {
    pub federation: Federation,
    pub country: Country,
    pub state: String,
    pub town: String,
    pub name: String,
}
