use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use std::time::{ Duration, SystemTime, UNIX_EPOCH };
use chrono::format;
use ratelimit::Ratelimiter;
use redis::Commands;
use thiserror::Error;
use http_body_util::{ BodyExt, Full };
use hyper::{ body::Bytes, client::conn::http1, Request, Response };
use tokio::{ net::TcpStream, spawn, time::timeout };
use crate::server::RateLimiterMap;
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
    #[error("configuration loading error")]
    ConfigError,
    #[error("invalid host header")]
    HostError,
    #[error("no forwarding rules configured")]
    NoForwardingRules,
    #[error("forwarding rule not found")]
    RuleNotFound,
    #[error("connection error: {0}")] ConnectionError(String),
    #[error("HTTP communication error")]
    HttpCommError,
    #[error("Redis error: {0}")] RedisError(#[from] redis::RedisError),
}

pub async fn handle_http_connections(
    req: Request<hyper::body::Incoming>,
    client_ip: String,
    rate_limiter_map: &RateLimiterMap
) -> Result<Response<Full<Bytes>>, ProxyError> {
    log::info!("Client IP: {}", client_ip);

    // let mut rate_limiters = rate_limiter_map.lock().await;
    // // see all the rate limiters values and keys

    // rate_limiters.iter().for_each(|(key, value)| {
    //     println!("Key: {}, Value: {:?}", key, value.rate());
    // });
    // let rate_limiter = rate_limiters
    //     .entry(client_ip.clone())
    //     .or_insert_with(|| {
    //         Ratelimiter::builder(10, Duration::from_secs(60)).max_tokens(10).build().unwrap()
    //     });

    // // Apply rate limiting
    // if let Err(sleep) = rate_limiter.try_wait() {
    //     // Rate limit exceeded

    //     if let Err(sleep) = rate_limiter.try_wait() {
    //         let title = "<h1>429 Too Many Requests</h1>";
    //         let message = format!(
    //             "p>Rate limit exceeded. Try again in {} seconds</p>",
    //             sleep.as_secs()
    //         );
    //         return Ok(show_html_page(title, &message));
    //     }
    // }

    enforce_rate_limit(&req, &client_ip, rate_limiter_map)?;

    let configs = load_configs().map_err(|_| ProxyError::ConfigError)?;
    log::debug!("Configs: {:?}", configs);

    // if tls is enabled, return error with this site  is not secure , if you are the owner of this website, please configure it properly or if you are a visitor, please try again later

    // Extract the host from the request
    let host = extract_host(&req).map_err(|_| ProxyError::HostError)?;

    log::debug!("Host: {:?}", host);

    // Ensure forwarding rules are present
    let forwarding_rules = configs.forwarding_rules;
    if forwarding_rules.is_none() {
        if configs.static_files_directory.is_none() {
            return Ok(show_default_page());
        } else {

            log::debug!("looking at statics file in the configs");
            let file = fs
                ::read_to_string(configs.static_files_directory.unwrap())
                .map_err(|_| ProxyError::RuleNotFound)?;
            return Ok(
                Response::builder()
                    .status(404)
                    .body(Full::from(Bytes::from(file)))
                    .unwrap()
            );
        }
    }

    // Get the forwarding rules
    let forwarding_rules = forwarding_rules.ok_or(ProxyError::NoForwardingRules)?;

    log::debug!("Forwarding Rules: {:?}", forwarding_rules);

    println!("{:?}", forwarding_rules);
    // Get the forwarding rule for the host
    let rule = get_forwarding_rule(&Some(forwarding_rules), &host).map_err(
        |_| ProxyError::RuleNotFound
    )?;

    log::debug!("Forwarding Rule: {:?}", rule);

    // Connect to the destination server with a timeout
    let destination = rule.destination;
    log::debug!("Destination: {:?}", destination);

    let stream = timeout(Duration::from_secs(10), TcpStream::connect(destination)).await
        .map_err(|_| ProxyError::ConnectionError("Connection timed out".to_string()))?
        .map_err(|e| ProxyError::ConnectionError(e.to_string()))?;

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

    Ok(Response::from_parts(parts, final_body))
}

fn create_error_response(status_code: u16, message: &str, title: &str) -> Response<Full<Bytes>> {
    let response = HttpMessageError {
        status_code,
        message: message.to_string(),
        title: title.to_string(),
    };
    log::error!("{}", message);
    let client_message =
        "We encountered an internal server error. Please contact the admin if this persists.";
    http_error_response(
        response.status_code,
        client_message.to_string(),
        response.title
    ).unwrap_or_else(|_| {
        Response::builder()
            .status(500)
            .body(Full::from(Bytes::from("Internal Server Error")))
            .unwrap()
    })
}

fn show_default_page() -> Response<Full<Bytes>> {
    let file = fs::read_to_string("/etc/sheldx/static/index.html").unwrap();
    log::debug!("Default page: {:?}", file);
    Response::builder()
        .status(404)
        .body(Full::from(Bytes::from(file)))
        .unwrap()
}
pub  fn show_html_page(title: &str, message: &str) -> Response<Full<Bytes>> {
    let html = format!(
        "<!DOCTYPE html>
        <html>
            <head>
                <title>{}</title>
            </head>
            <body>
                <h1>{}</h1>
                <p>{}</p>
            </body>
        </html>",
        title,
        title,
        message
    );
    Response::builder()
        .status(404)
        .body(Full::from(Bytes::from(html)))
        .unwrap()
}












    // let client = redis::Client::open("redis://127.0.0.1:6383")?;
    // let mut con = client.get_connection()?;

    // let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    // let key_exists: bool = con.exists(&client_ip)?;

    // if key_exists {
    //     let request_count: u32 = con.get(&client_ip)?;
    //     if request_count > 10 {
    //         //set Retry-After header
    //         let time_remaining: i32 = 60 ;
    //         let resposne =Response::builder()
    //             .status(429)
    //             .header("Retry-After", time_remaining.to_string())
    //             .body(Full::from(Bytes::from("Too many requests")))
    //             .unwrap();
    //         return Ok(resposne);

    //     } else {
    //         con.incr::<&str, u32, u32>(&client_ip, 1)?;
    //     }
    // } else {
    //     con.set::<&str, u32, ()>(&client_ip, 1)?;
    //     con.expire::<&str, usize>(&client_ip, 60)?;

    // we using hashmaps because hashmaps is O(1) and we can use it to store the rate limiters for each client ip
