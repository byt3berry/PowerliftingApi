use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Eq, EnumIter, PartialEq)]
pub enum CountryDto {
    FRANCE,
    OTHER,
}
