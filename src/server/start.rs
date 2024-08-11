use std::convert::Infallible;
use std::fs::File;
use std::io::BufReader;
use std::net::{ IpAddr, Ipv4Addr };
use std::path::PathBuf;
use std::{ error::Error as StdError, net::SocketAddr };
use std::sync::Arc;
use async_trait::async_trait;
use http_body_util::{ BodyExt, Full };
use hyper::body::{ Body, Buf, Bytes, Incoming };
use hyper::client::conn::http1::{ self, handshake, Connection, SendRequest };
use hyper::server::conn::http1::{ self as serverhttp, Builder };
use hyper::service::service_fn;
use hyper::{ Method, Request, Response, StatusCode };
use hyper_util::rt::TokioIo;
use log;
use log4rs::config;
use rustls::pki_types::{ CertificateDer, PrivateKeyDer };
use rustls_pemfile::pkcs8_private_keys;
use tokio::fs;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::{ TcpListener, TcpStream };
use tokio_rustls::TlsAcceptor;
use std::future::Future;
use std::pin::Pin;

use crate::handlers::handle_http_connections;
use crate::utils::{ init_logger, load_configs, Configs };

/// A struct representing a server that does not use TLS.
/// It contains a connection handler function that will be called
/// for each incoming TCP connection.
pub struct WithoutTLS {}
pub struct WithTls {}

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

        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 443);
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

#[async_trait]
impl Server for WithTls {
    async fn start(&self) -> Result<(), Box<dyn StdError>> {
        let configs = load_configs()?;
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let keyfile = configs.key_path;
        let certfile = configs.cert_path;

        let cert_path = PathBuf::from(certfile);
        let key_path = PathBuf::from(keyfile);

        let mut cert_file = BufReader::new(File::open(cert_path)?);
        let mut key_file = BufReader::new(File::open(key_path)?);

        // Load certificates
        let certs = rustls_pemfile::certs(&mut cert_file).collect::<Result<Vec<_>, _>>()?; // Use `?` for error handling
        let private_key = rustls_pemfile::private_key(&mut key_file)?; // Use `?` for error handling
        log::debug!("Certificates: {:?}", certs);
        log::debug!("Private Key: {:?}", private_key);
        let config = rustls::ServerConfig
            ::builder()
            .with_no_client_auth()
            .with_single_cert(certs, private_key.unwrap())
            .unwrap();

        let acceptor = TlsAcceptor::from(Arc::new(config.clone()));

        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 443);
        let listener = TcpListener::bind(addr).await?;
        let service = service_fn(handle_http_connections);

        log::info!("Starting server on: {}", addr);
        loop {
            let (stream, _) = listener.accept().await?;

            let tls_acceptor = acceptor.clone();
            tokio::task::spawn(async move {
                let tls_stream = match tls_acceptor.accept(stream).await {
                    Ok(tls_stream) => {
                        log::info!("TLS handshake successful");
                        
                        tls_stream},
                    Err(err) => {
                        eprintln!("failed to perform tls handshake: {err:#}");
                        return;
                    }
                };
                if
                    let Err(err) = Builder::new().serve_connection(
                        TokioIo::new(tls_stream),
                        service.clone()
                    ).await
                {
                    eprintln!("failed to serve connection: {err:#}");
                }
            });
        }

        Ok(())
    }
}
async fn echo(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let mut response = Response::new(Full::default());
    match (req.method(), req.uri().path()) {
        // Help route.
        (&Method::GET, "/") => {
            *response.body_mut() = Full::from("Try POST /echo\n");
        }
        // Echo service route.
        (&Method::POST, "/echo") => {
            *response.body_mut() = Full::from(req.into_body().collect().await?.to_bytes());
        }
        // Catch-all 404.
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }
    Ok(response)
}
