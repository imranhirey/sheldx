use std::collections::HashMap;
use std::fs::{ self, File };
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;

use http_body_util::{ BodyExt, Full };
use hyper::{ body::Bytes, client::conn::http1, Request, Response };
use ratelimit::Ratelimiter;
use thiserror::Error;
use tokio::sync::Mutex;
use tokio::{ net::TcpStream, spawn, time::timeout };

use crate::services::enforce_rate_limit;
use crate::utils::{
  extract_host,
  get_forwarding_rule,
  http_error_response,
  load_configs,
  HttpMessageError,
};

#[derive(Error, Debug)]
pub enum ProxyError {
  #[error("Configuration loading error")]
  ConfigError,
  #[error("Invalid host header")]
  HostError,
  #[error("No forwarding rules configured")]
  NoForwardingRules,
  #[error("Forwarding rule not found")]
  RuleNotFound,
  #[error("Connection error: {0}")] ConnectionError(String),
  #[error("HTTP communication error")]
  HttpCommError,
  #[error("Redis error: {0}")] RedisError(#[from] redis::RedisError),
}

pub async fn handle_http_connections(
  req: Request<hyper::body::Incoming>,
  client_ip: String,
  rate_limiter_map: Arc<Mutex<HashMap<String, Ratelimiter>>>
) -> Result<Response<Full<Bytes>>, ProxyError> {
  log::debug!("Client IP: {:?}", client_ip);
  let connection_timeout: Duration = Duration::from_secs(5);
  let max_retries: u8 = 3;
  let host = extract_host(&req).map_err(|_| ProxyError::HostError)?;
  log::debug!("Host: {:?}", host);

  let configs = load_configs().map_err(|_| ProxyError::ConfigError)?;
  log::debug!("Configs: {:?}", configs);

  let rate_limit_status = enforce_rate_limit(&req, &client_ip, &rate_limiter_map, &configs).await?;
  log::debug!("Rate limit status: {:?}", rate_limit_status.response);

  if rate_limit_status.status_code != 200 {
    let html_content = read_file_content("/etc/sheldx/static/rate_limit.html").map_err(
      |_| ProxyError::RuleNotFound
    )?;
    let final_html = html_content
      .replace("{{user_ip}}", &client_ip)
      .replace("{{seconds}}", &rate_limit_status.seconds.to_string());

    return Ok(
      Response::builder()
        .status(rate_limit_status.status_code)
        .body(Full::from(Bytes::from(final_html)))
        .unwrap()
    );
  }

  // Handle static files or error responses if no forwarding rules are configured
  let forwarding_rules = configs.forwarding_rules;
  if forwarding_rules.is_none() {
    if let Some(static_files_directory) = configs.static_files_directory {
      log::debug!("Serving static file from directory: {}", static_files_directory);
      let file_content = fs
        ::read_to_string(static_files_directory)
        .map_err(|_| ProxyError::RuleNotFound)?;
      return Ok(
        Response::builder()
          .status(404)
          .body(Full::from(Bytes::from(file_content)))
          .unwrap()
      );
    } else {
      return Ok(show_default_page());
    }
  }

  // Get the forwarding rule for the host
  let forwarding_rules = forwarding_rules.ok_or(ProxyError::NoForwardingRules)?;
  log::debug!("Forwarding Rules: {:?}", forwarding_rules);

  let rule = get_forwarding_rule(&Some(forwarding_rules), &host).map_err(
    |_| ProxyError::RuleNotFound
  )?;

  let destination = rule.target;
  log::debug!("Destination: {:?}", destination);

  let mut last_error = None;
  for attempt in 1..=max_retries {
    match timeout(connection_timeout, TcpStream::connect(&destination)).await {
      Ok(Ok(stream)) => {
        let io = hyper_util::rt::TokioIo::new(stream);
        let (mut send_request, connection) = http1
          ::handshake(io).await
          .map_err(|_| ProxyError::HttpCommError)?;

        spawn(async move {
          if let Err(err) = connection.await {
            log::error!("Error serving connection: {:?}", err);
          }
        });

        let res = send_request.send_request(req).await.map_err(|_| ProxyError::HttpCommError)?;
        let (parts, body) = res.into_parts();
        let bytes = body.collect().await.map_err(|_| ProxyError::HttpCommError)?;
        let final_body: Full<Bytes> = Full::from(bytes.to_bytes());

        return Ok(Response::from_parts(parts, final_body));
      }
      Ok(Err(e)) => {
        last_error = Some(ProxyError::ConnectionError(e.to_string()));
        log::warn!("Attempt {} to connect to {} failed: {}", attempt, destination, e);
      }
      Err(_) => {
        last_error = Some(ProxyError::ConnectionError("Connection timed out".to_string()));
        log::warn!("Attempt {} to connect to {} timed out", attempt, destination);
      }
    }

    if attempt < max_retries {
      log::info!("Retrying connection to {} (attempt {})", destination, attempt + 1);
    }
  }

 Ok(show_internal_server_error())
}

fn read_file_content(path: &str) -> Result<String, std::io::Error> {
  let mut file = File::open(path)?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;
  Ok(content)
}

fn show_default_page() -> Response<Full<Bytes>> {
  let file_content = read_file_content("/etc/sheldx/static/index.html").unwrap_or_else(|_|
    "Default page not found".to_string()
  );
  log::debug!("Default page: {:?}", file_content);

  Response::builder()
    .status(404)
    .body(Full::from(Bytes::from(file_content)))
    .unwrap()
}



fn show_internal_server_error() -> Response<Full<Bytes>> {
 let html_content = read_file_content("/etc/sheldx/static/internal_server_error.html").unwrap_or_else(|_| {
     "<h1>Internal Server Error</h1>".to_string()
 });
  Response::builder()
      .status(500)
      .body(Full::from(Bytes::from(html_content)))
      .unwrap()

}