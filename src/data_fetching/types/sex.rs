#[derive(Debug, Default)]
pub enum Sex {
    #[default]
    M,
    F,
    /// A gender-neutral title, including non-binary lifters.
    Mx,
}
