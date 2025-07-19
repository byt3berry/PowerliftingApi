use super::weight::Weight;

/// The definition of the "WeightClassKg" column.
#[derive(Debug, Default)]
pub enum WeightClass {
    /// A class defined as being under or equal to a maximum weight.
    UnderOrEqual(Weight),
    /// A class defined as being over a minimum weight, for superheavies.
    Over(Weight),
    /// No weight class information supplied.
    #[default]
    None,
}
