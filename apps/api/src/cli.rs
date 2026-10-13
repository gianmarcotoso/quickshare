pub mod init;
pub mod openapi;

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "quickshare", version = crate::core::version::VERSION)]
pub struct Cli {
    #[arg(
        long,
        global = true,
        env = "QUICKSHARE_CONFIG",
        default_value = "config.ini"
    )]
    pub config: PathBuf,

    #[arg(long, help = "Listen address, overrides Server.Bind")]
    pub bind: Option<String>,

    #[arg(long, help = "Listen port, overrides Server.Port")]
    pub port: Option<u16>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Init,
    Openapi,
}

pub fn run(config: &Path, command: Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::Init => init::init(config),
        Command::Openapi => openapi::openapi(),
    }
}
