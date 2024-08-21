use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::net::{ IpAddr, Ipv4Addr, SocketAddr };
use std::{ error::Error as StdError, sync::Arc };
use async_trait::async_trait;
use hyper::server::conn::http1 as http1_serevr;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use rustls::ServerConfig;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_rustls::TlsAcceptor;

use crate::handlers::{ handle_http_connections, handle_https_connections };
use crate::utils::load_configs;
use ratelimit::Ratelimiter;

pub type RateLimiterMap = Arc<Mutex<HashMap<String, Ratelimiter>>>;

/// A struct representing a server that does not use TLS.
pub struct WithoutTLS {
  pub port: Option<u16>, // Default port for HTTP is 80, but user can change it
}
pub struct WithTLS {
  pub port: Option<u16>, // Default port for HTTPS is 443, but user can change it
}

pub enum PORTS {
  HTTP = 8080,
  HTTPS = 443,
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
    // chheck if tehre is atleast one ule

    log::warn!("Sheldx recommends using TLS for production use");
    let port = self.port.unwrap_or(PORTS::HTTP as u16);
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
        if
          let Err(err) = http1_serevr::Builder
            ::new()
            .auto_date_header(true)
            .half_close(true)
            .serve_connection(
              io,
              service_fn(move |req| {
                let rate_limiter_map = rate_limiter_map.clone();
                handle_http_connections(req, client_ip.clone(), rate_limiter_map)
              })
            ).await
        {
          // show 500 error
          log::error!("Error serving connection: {:?}", err);
        }
      });
    }
  }
}

#[async_trait]
impl Server for WithTLS {
  async fn start(&self) -> Result<(), Box<dyn StdError>> {
    // setup the keys

    let configs = load_configs()?;
    let port = self.port.unwrap_or(PORTS::HTTPS as u16);
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 443);

    log::info!("Starting server on: {}", addr);

    let mut cert_path = configs.cert_path.clone();
    let mut key_path = configs.key_path.clone();

    let cert_file = &mut BufReader::new(File::open(cert_path).unwrap());
    let key_file = &mut BufReader::new(File::open(key_path).unwrap());

    let certs = rustls_pemfile
      ::certs(&mut BufReader::new(cert_file))
      .collect::<Result<Vec<_>, _>>()
      .unwrap();
    let private_key = rustls_pemfile::private_key(&mut BufReader::new(key_file)).unwrap().unwrap();

    let config: Result<ServerConfig, rustls::Error> = rustls::ServerConfig
      ::builder()
      .with_no_client_auth()
      .with_single_cert(certs, private_key);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", 4444)).await.unwrap();
    let tls_acceptor = TlsAcceptor::from(Arc::new(config.unwrap()));
    let rate_limiter_map: RateLimiterMap = Arc::new(Mutex::new(HashMap::new()));

    loop {
      let (mut stream, _) = listener.accept().await?;
      let client_ip = stream.peer_addr()?.ip().to_string();
      log::info!("Accepted connection from: {}", client_ip);

      let rate_limiter_map = rate_limiter_map.clone();

      // write simple http sresponse i am from sheldx https server

      let mut https_stream = tls_acceptor.accept(stream).await.unwrap();
      let io = TokioIo::new(https_stream);
      tokio::spawn(async move {
        if
          let Err(err) = http1_serevr::Builder
            ::new()
            .auto_date_header(true)
            .half_close(true)
            .serve_connection(
              io,
              service_fn(move |req| {
                
                  let rate_limiter_map = rate_limiter_map.clone();
               handle_http_connections(req, client_ip.clone(), rate_limiter_map)
                
                 })
            ).await
        {
          // show 500 error
          log::error!("Error serving connection: {:?}", err);
        }
      });
    }
  }
}
