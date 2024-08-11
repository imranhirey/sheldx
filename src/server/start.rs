use std::convert::Infallible;
use std::net::{ IpAddr, Ipv4Addr };
use std::{ error::Error as StdError, net::SocketAddr };
use std::sync::Arc;
use async_trait::async_trait;
use http_body_util::{ BodyExt, Full };
use hyper::body::{ Body, Bytes, Incoming };
use hyper::client::conn::http1::{ self, handshake, Connection, SendRequest };
use hyper::server::conn::http1 as serverhttp;
use hyper::service::service_fn;
use hyper::{ Request, Response };
use hyper_util::rt::TokioIo;
use log;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::{ TcpListener, TcpStream };
use std::future::Future;
use std::pin::Pin;

use crate::handlers::handle_http_connections;
use crate::utils::{ init_logger, load_configs };

/// A struct representing a server that does not use TLS.
/// It contains a connection handler function that will be called
/// for each incoming TCP connection.
pub struct WithoutTLS {}

/// Trait representing a server that can be started.
///
/// Implementations of this trait should provide a way to start
/// the server and handle incoming connections.
#[async_trait]
pub trait Server {
    /// Starts the server and begins accepting incoming connections.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the server started successfully, or
    /// an error if there was an issue.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to start
    /// the server or handle incoming connections.
    async fn start(&self) -> Result<(), Box<dyn StdError>>;
}

#[async_trait]
impl Server for WithoutTLS {
    async fn start(&self) -> Result<(), Box<dyn StdError>> {
        let configs = load_configs()?;

        // Inform the user that TLS is recommended for production use
        log::warn!("Sheldx recommends using TLS for production use");

        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);
        log::info!("Starting server on: {}", addr);
        let listener = TcpListener::bind(&addr).await?;
        log::info!("Server started on: {}", addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);

            if configs.show_logs_on_console {
                log::info!("Accepted connection from: {}", addr);
            }

            tokio::spawn(async move {
                if
                    let Err(err) = serverhttp::Builder
                        ::new()
                        .serve_connection(io, service_fn(handle_http_connections)).await
                {
                    log::error!("Error while serving connection: {}", err);
                }
            });
        }
    }
}
