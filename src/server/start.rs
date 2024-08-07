use std::error::Error as StdError;
use std::sync::Arc;
use async_trait::async_trait;
use log;
use tokio::net::{TcpListener, TcpStream};
use std::future::Future;
use std::pin::Pin;

use crate::utils::{init_logger, load_configs};

/// A struct representing a server that does not use TLS.
/// It contains a connection handler function that will be called
/// for each incoming TCP connection.
pub struct WithoutTLS {
    /// A connection handler function that processes each incoming
    /// `TcpStream`. The function is expected to return a future
    /// that resolves to a `Result<(), Box<dyn StdError + Send + Sync>>`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tokio::net::TcpStream;
    /// use std::future::Future;
    /// use std::pin::Pin;
    /// use std::sync::Arc;
    /// use std::error::Error;
    /// 
    /// async fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {
    ///     // Handle the connection
    ///     Ok(())
    /// }
    /// 
    /// let handler: Arc<dyn Fn(TcpStream) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send>> + Send + Sync> = Arc::new(|stream| Box::pin(handle_connection(stream)));
    /// ```
    pub connection_handler: Arc<dyn Fn(TcpStream) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn StdError + Send + Sync>>> + Send>> + Send + Sync>,
}

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

        let addr = "0.0.0.0:3000";
        log::info!("Starting server on: {}", addr);
        let listener = TcpListener::bind(&addr).await?;
        log::info!("Server started on: {}", addr);

        loop {
            let (socket, _) = listener.accept().await?;
            
            if configs.show_logs_on_console {
                log::info!("Accepted connection from: {}", socket.peer_addr()?);
            }
            
            let connection_handler = Arc::clone(&self.connection_handler);
            
            tokio::spawn(async move {
                if let Err(e) = connection_handler(socket).await {
                    log::error!("Failed to handle connection: {}", e);
                }
            });
        }
    }
}
