use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{error::Error as StdError, sync::Arc};
use async_trait::async_trait;
use hyper::server::conn::http1 as serverhttp;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::handlers::handle_http_connections;
use crate::utils::load_configs;
use ratelimit::Ratelimiter;

pub type RateLimiterMap = Arc<Mutex<HashMap<String, Ratelimiter>>>;

/// A struct representing a server that does not use TLS.
pub struct WithoutTLS {
    pub port: Option<u16>, // Default port for HTTP is 80, but user can change it
}

#[async_trait]
pub trait Server {
    /// Starts the server and begins accepting incoming connections.
    async fn start(&self) -> Result<(), Box<dyn StdError>>;
}

#[async_trait]
impl Server for WithoutTLS {
    async fn start(&self) -> Result<(), Box<dyn StdError>> {
        let configs = load_configs()?;

        log::warn!("Sheldx recommends using TLS for production use");
        let port = self.port.unwrap_or(80);
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

        log::info!("Starting server on: {}", addr);
        let listener = TcpListener::bind(&addr).await.map_err(|e| {
            log::error!("Failed to bind to address: {}", e);
            e
        })?;

        let rate_limiter_map: RateLimiterMap = Arc::new(Mutex::new(HashMap::new()));

        loop {
            let (stream, _) = listener.accept().await?;
            let client_ip = stream.peer_addr()?.ip().to_string();
            log::info!("Accepted connection from: {}", client_ip);

            let rate_limiter_map = rate_limiter_map.clone();
            let io = TokioIo::new(stream);

            if configs.show_logs_on_console {
                log::info!("Accepted connection from: {}", client_ip);
            }

            tokio::spawn(async move {
                if let Err(err) = serverhttp::Builder::new()
                    .serve_connection(io, service_fn(move |req| {
                        let rate_limiter_map = rate_limiter_map.clone();
                        handle_http_connections(req, client_ip.clone(), rate_limiter_map)
                    }))
                    .await
                {
                    log::error!("Error while serving connection: {}", err);
                }
            });
        }
    }
}
