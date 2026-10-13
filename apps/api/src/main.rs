mod app;
mod cli;
mod core;
mod features;
mod libs;
mod openapi;
mod router;

#[cfg(test)]
mod tests;

use clap::Parser;

use crate::{
    cli::Cli,
    core::{app_config::AppConfig, logging::init_logging},
    libs::sha256_file::sha256_file,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => cli::run(&cli.config, command),
        None => {
            let mut config = AppConfig::load(&cli.config)?;

            if let Some(bind) = cli.bind {
                config.bind = bind;
            }

            if let Some(port) = cli.port {
                config.port = port;
            }

            init_logging(&config.log_level);

            let executable_sha256 = sha256_file(&std::env::current_exe()?)?;

            app::start(app::init(config, executable_sha256)).await
        }
    }
}
