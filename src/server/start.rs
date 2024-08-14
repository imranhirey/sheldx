use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::net::{ IpAddr, Ipv4Addr };
use std::path::PathBuf;
use std::time::Duration;
use std::{ error::Error as StdError, net::SocketAddr };
use std::sync::Arc;
use async_trait::async_trait;
use hyper::server::conn::http1::{ self as serverhttp, Builder };
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use log;

use ratelimit::Ratelimiter;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_rustls::TlsAcceptor;

use crate::handlers::handle_http_connections;
use crate::utils::load_configs;

/// A struct representing a server that does not use TLS.
/// It contains a connection handler function that will be called
/// for each incoming TCP connection.
pub struct WithoutTLS {}
pub struct WithTls {}

/// Trait representing a server that can be started.
///
/// Implementations of this trait should provide a way to start
/// the server and handle incoming connections.
pub  type RateLimiterMap = Arc<Mutex<HashMap<String, Ratelimiter>>>;
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

        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3001);
        log::info!("Starting server on: {}", addr);
        let listener = TcpListener::bind(&addr).await?;
        log::info!("Server started on: {}", addr);
        // use RateLimiterMap = Arc<Mutex<HashMap<String, Ratelimiter>>>;

        let rate_limiter_map: RateLimiterMap = Arc::new(Mutex::new(HashMap::new()));        

        loop {
            let (stream, _) = listener.accept().await?;
                 // get teh users real ip address
                 log::info!("Accepted connection from: {}", stream.peer_addr()?);
                 // print the rate limiter map entries
                 
                   

        let client_ip = stream.peer_addr()?.ip().to_string();
        let rate_limiter_map = rate_limiter_map.clone();
            
            let io = TokioIo::new(stream);

            if configs.show_logs_on_console {
                log::info!("Accepted connection from: {}", addr);
            }


            tokio::spawn(async move {
                if
                    let Err(err) = serverhttp::Builder
                        ::new()
                        .serve_connection(io, service_fn(
                            
                            |req| handle_http_connections(req, client_ip.clone(),
                            &rate_limiter_map)
                        )).await
                {
                    log::error!("Error while serving connection: {}", err);
                }
            });
        }
    }
}

// #[async_trait]
// impl Server for WithTls {
//     async fn start(&self) -> Result<(), Box<dyn StdError>> {
//         let configs = load_configs()?;
//         let keyfile = configs.key_path;
//         let certfile = configs.cert_path;

//         let cert_path = PathBuf::from(certfile);
//         let key_path = PathBuf::from(keyfile);

//         let mut cert_file = BufReader::new(File::open(cert_path)?);
//         let mut key_file = BufReader::new(File::open(key_path)?);

//         // Load certificates
//         let certs = rustls_pemfile::certs(&mut cert_file).collect::<Result<Vec<_>, _>>()?; // Use `?` for error handling
//         let private_key = rustls_pemfile::private_key(&mut key_file)?; // Use `?` for error handling
//         log::debug!("Certificates: {:?}", certs);
//         log::debug!("Private Key: {:?}", private_key);
//         let config = rustls::ServerConfig
//             ::builder()
//             .with_no_client_auth()
//             .with_single_cert(certs, private_key.unwrap())
//             .unwrap();

//         let acceptor = TlsAcceptor::from(Arc::new(config.clone()));
//         let _rate_limiter = Arc::new(Ratelimiter::builder( 10, Duration::from_secs(60)).build().unwrap());


//         let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 443);
//         let listener = TcpListener::bind(addr).await?;
//         let service = service_fn(
            
//             |req| handle_http_connections(req, String::new(), Arc::new(Ratelimiter::builder( 10, Duration::from_secs(60)).build().unwrap()))
//         );

//         log::info!("Starting server on: {}", addr);
//         loop {
//             let (stream, _) = listener.accept().await?;

//             let tls_acceptor = acceptor.clone();
//             tokio::task::spawn(async move {
//                 let tls_stream = match tls_acceptor.accept(stream).await {
//                     Ok(tls_stream) => {
//                         log::info!("TLS handshake successful");

//                         tls_stream
//                     }
//                     Err(err) => {
//                         eprintln!("failed to perform tls handshake: {err:#}");
//                         return;
//                     }
//                 };
//                 if
//                     let Err(err) = Builder::new().serve_connection(
//                         TokioIo::new(tls_stream),
//                         service.clone()
//                     ).await
//                 {
//                     log::error!("failed to serve connection: {err:#}");
//                 }
//             });
//         }
//     }
// }
