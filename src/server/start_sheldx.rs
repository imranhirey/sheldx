
use crate::{server::PORTS, utils::{load_configs, start_redis}};
use super::{Server, WithoutTLS};
use std::error::Error;

pub async fn start_sheldx() -> Result<(), Box<dyn Error>> {
    let configs = load_configs()?;

    if configs.is_tls_enabled {
        // TLS support not implemented yet
        log::error!("TLS is not supported yet. Please use non-TLS mode or implement TLS support.");
        return Err("TLS is not supported yet".into());
    }

    // Log a warning about the lack of TLS support in production
    log::warn!("Sheldx recommends using TLS for production use.");

    // Start the server without TLS
    let server = WithoutTLS { port: Some(PORTS::HTTP as u16) };
    if let Err(e) = server.start().await {
        log::error!("Error starting server: {}", e);
        return Err(e.into());
    }

    // Attempt to start Redis, with fallback to in-memory cache if it fails
    if let Err(e) = start_redis() {
        log::error!("Error starting Redis: {}", e);
        log::info!("Switching to in-memory cache as a fallback.");
    }

    log::info!("Sheldx started successfully.");
    Ok(())
}
