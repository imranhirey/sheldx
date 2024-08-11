use std::{fs, time::Duration};
use thiserror::Error;
use http_body_util::{ BodyExt, Full };
use hyper::{ body::{ Buf, Bytes, Incoming }, client::conn::http1, Request, Response };
use tokio::{ net::TcpStream, spawn, time::timeout };
use crate::utils::{
    extract_host,
    get_forwarding_rule,
    http_error_response,
    load_configs,
    HttpMessageError,
};

// Custom error type for more descriptive error handling
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
    HttpCommError
}

pub async fn handle_http_connections(
    req: Request<Incoming>
) -> Result<Response<Full<Bytes>>, ProxyError> {
    // Helper function to create an error response

    // Load the configs
    let configs = load_configs().map_err(|_| ProxyError::ConfigError)?;

    log::debug!("Configs: {:?}", configs);

    // if tls is enabled, return error with this site  is not secure , if you are the owner of this website, please configure it properly or if you are a visitor, please try again later
    
    // Extract the host from the request
    let host = extract_host(&req).map_err(|_| ProxyError::HostError)?;

    log::debug!("Host: {:?}", host);

    // Ensure forwarding rules are present
    let forwarding_rules = configs.forwarding_rules;
 if  forwarding_rules.is_none() {
        if configs.static_files_directory.is_none() {
            return Ok(show_default_page());
        } else {
            let file = fs::read_to_string(configs.static_files_directory.unwrap()).map_err(
                |_| ProxyError::RuleNotFound
            )?;
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

    // Create a new connection
    let io = hyper_util::rt::TokioIo::new(stream);
    let (mut send_request, connection) = http1
        ::handshake(io).await
        .map_err(|_| ProxyError::HttpCommError)?;

    spawn(async move {
        if let Err(err) = connection.await {
            log::error!("Error serving connection: {:?}", err);
        }
    });

    // Send the request to the destination server
    let res = send_request.send_request(req).await.map_err(|_| ProxyError::HttpCommError)?;

    let (parts, body) = res.into_parts();

    // Convert the body to bytes
    let bytes = body.collect().await.map_err(|_| ProxyError::HttpCommError)?;
    let final_body: Full<Bytes> = Full::from(bytes.to_bytes());

    let response = Response::from_parts(parts, final_body);
    Ok(response)
}

impl From<ProxyError> for Response<Full<Bytes>> {
    fn from(err: ProxyError) -> Self {
        match err {
            ProxyError::ConfigError =>
                create_error_response(
                    500,
                    "We encountered an internal server error while loading configurations.",
                    "Internal Server Error"
                ),
            ProxyError::HostError =>
                create_error_response(400, "The host header is invalid.", "Bad Request"),
            ProxyError::NoForwardingRules =>
                create_error_response(404, "No forwarding rules are configured.", "Not Found"),
            ProxyError::RuleNotFound =>
                create_error_response(
                    404,
                    "The requested URL was not found on this server.",
                    "Not Found"
                ),
            ProxyError::ConnectionError(_) =>
                create_error_response(
                    500,
                    "Failed to connect to the destination server.",
                    "Internal Server Error"
                ),
            ProxyError::HttpCommError =>
                create_error_response(
                    500,
                    "Failed to communicate with the destination server.",
                    "Internal Server Error"
                ),
        }
    }
}
fn create_error_response(status_code: u16, message: &str, title: &str) -> Response<Full<Bytes>> {
    let response = HttpMessageError {
        status_code,
        message: message.to_string(),
        title: title.to_string(),
    };
    log::error!("{}", message);
    // Send a generic error message to the client
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

// show default page

fn show_default_page() -> Response<Full<Bytes>> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory");
    let file= fs::read_to_string(home_dir.unwrap().to_str().unwrap().to_string() + "/sheldx/statics/index.html").unwrap();
    log::debug!("Default page: {:?}", file);
    Response::builder()
        .status(404)
        .body(Full::from(Bytes::from(file)))
        .unwrap()


 
}