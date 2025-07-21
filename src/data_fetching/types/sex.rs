use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
pub enum Sex {
    #[default]
    M,
    F,
    /// A gender-neutral title, including non-binary lifters.
    Mx,
}
