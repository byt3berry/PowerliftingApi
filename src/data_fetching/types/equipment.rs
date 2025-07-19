#[derive(Debug, Default)]
pub enum Equipment {
    Raw,
    Wraps,
    Single,
    Multi,
    #[default]
    Unlimited,
    Straps,
}
