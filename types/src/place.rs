use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub enum Place {
    /// The placing assigned to the entry.
    P(u8),
    /// Guest Lifter.
    G,
    /// Disqualified.
    DQ,
    /// Doping Disqualification.
    DD,
    /// No-Show.
    #[default]
    NS,
}
