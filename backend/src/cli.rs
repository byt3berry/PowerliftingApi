use anyhow::{bail, Result};
use std::net::IpAddr;
use std::path::PathBuf;

use clap::Parser;

/// `PowerliftingApi`
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to data
    #[arg(long, env = "DATA")]
    pub path: Option<PathBuf>,

    /// IP
    #[arg(long, env = "IP", requires = "port")]
    pub ip: Option<IpAddr>,

    /// Port
    #[arg(long, env = "PORT", requires = "ip")]
    pub port: Option<u16>,

    /// Apply migrations
    #[arg(short, long, default_value = "true")]
    pub start_server: Option<bool>,

    /// Apply migrations
    #[arg(short, long, default_value = "true")]
    pub migrate: Option<bool>,
}

impl Args {
    pub fn validate(&self) -> Result<()> {
        if self.path.as_ref().is_none_or(|path| !path.exists()) {
            bail!("path \"{:?}\" must exist", self.path);
        }

        if self.start_server.is_some_and(|start_server| start_server) {
            if self.ip.is_none() {
                bail!("ip must be set to start the server");
            }

            if self.port.is_none() {
                bail!("port must be set to start the server");
            }
        }

        Ok(())
    }
}
