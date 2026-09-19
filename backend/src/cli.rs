use anyhow::{Result, bail};
use std::net::IpAddr;
use std::path::PathBuf;

use clap::Parser;

/// `PowerliftingApi`
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to data
    #[arg(long, env = "DATA")]
    pub data_path: Option<PathBuf>,

    /// Path to static files
    #[arg(long, env = "STATIC_DATA")]
    pub static_path: PathBuf,

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
        if self.data_path.as_ref().is_none_or(|path| !path.exists()) {
            bail!("data path \"{:?}\" must exist", self.data_path);
        }

        if !self.static_path.exists() {
            bail!("static path \"{:?}\" must exist", self.static_path);
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
