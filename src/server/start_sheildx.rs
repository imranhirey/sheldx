use tokio::net::TcpStream;

use crate::handlers::handle_non_tls_connection;
use crate::utils::{ self, load_configs };
use std::error::Error as StdError;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use super::{ Server, WithoutTLS };

pub async fn start_sheildx() -> Result<(), Box<dyn StdError>> {
    let configs: utils::Configs = load_configs()?;
    let configs = Arc::new(configs); // Wrap configs in Arc
  if configs.is_tls_enabled {
        // validate if teh certificate and key paths are provided
        configs._check()?;
        // Start the server with TLS

        log::info!("Starting server with TLS enabled");
    }
   else{
    let server = WithoutTLS {
        connection_handler: Arc::new(move |socket: TcpStream| {
            let configs = Arc::clone(&configs); // Clone the Arc inside the closure
            Box::pin(handle_non_tls_connection(socket, configs)) as Pin<
                Box<dyn Future<Output = Result<(), Box<dyn StdError + Send + Sync>>> + Send>
            >
        }),
    };

    server.start().await?;
   }

    Ok(())
}
