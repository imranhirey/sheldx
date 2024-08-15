use crate::utils::{load_configs, start_redis};
use super::{Server, WithoutTLS};

pub async fn start_sheildx() -> Result<(), Box<dyn std::error::Error>> {
    let configs = load_configs()?;

    if configs.is_tls_enabled {
        // TLS support not implemented yet
        unimplemented!("TLS is not supported yet.");
    } else {
        log::warn!("Sheldx recommends using TLS for production use");
        let server = WithoutTLS {port:None};
        if let Err(e) = server.start().await {
            log::error!("Error starting server: {}", e);
            return Err(e.into());
        }
        start_redis().map_err(|e| {
            log::error!("Error starting Redis: {}", e);
            log::info!("we are setting the stretegy to use the in-memory cache");
            e
        })?;

        log::info!("Sheldx started successfully");


    }

    Ok(())
}
