use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum EquipmentDto {
    Any,
    Raw,
    Wraps,
    Single,
    Multi,
    Straps,
    Sleeves,
    Bare,
    Unlimited,
}
