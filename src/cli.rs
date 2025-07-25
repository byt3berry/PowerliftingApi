use anyhow::{bail, Result};
use std::net::IpAddr;
use std::path::PathBuf;

use clap::Parser;

/// `PowerliftingApi`
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// IP
    pub ip: IpAddr,

    /// Port
    pub port: u16,

    /// Path to data
    pub path: PathBuf,
}

impl Args {
    pub fn validate(&self) -> Result<()> {
        if !self.path.exists() {
            bail!("path must exist");
        }

        Ok(())
    }
}
