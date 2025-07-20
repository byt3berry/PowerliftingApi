use clap::Parser;

/// PowerliftingApi
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Port
    pub port: u16,
}
