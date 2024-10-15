use std::path::Path;

use cli::CliArgs;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

mod cli;
mod config;
#[cfg(test)]
mod test;
mod web;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("{APP_NAME} v{APP_VERSION} - developed by {APP_AUTHORS}");
    let config = config::Config::try_from_env()?;

    let cli_args = argh::from_env::<CliArgs>();

    if let Some(pidfile) = &cli_args.pidfile {
        write_pidfile(pidfile)?;
        info!("PID file written to {}", pidfile.display());
    }

    info!("initializing web service...");
    let web_service = web::WebServer::init(config.apikey, config.listener_addr).await?;
    info!("web service OK; running web server...");
    web_service.run().await?;

    Ok(())
}

/// Write PID to file
fn write_pidfile(p: &Path) -> anyhow::Result<()> {
    let pid = std::process::id();
    info!("Process started with PID {pid}");

    std::fs::write(p, pid.to_string())?;

    Ok(())
}
