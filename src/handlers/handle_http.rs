use std::fs::{self, File};
use std::io::Read;
use std::time::{ Duration };
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
    log::debug!("Client IP: {:?}", client_ip);
    let host = extract_host(&req).map_err(|_| ProxyError::HostError)?;

    log::debug!("Host waaa: {:?}", host);

    let configs = load_configs().map_err(|_| ProxyError::ConfigError)?;
    log::debug!("Configs: {:?}", configs);
    let rate_limit_status = enforce_rate_limit(&req, &client_ip, rate_limiter_map, &configs).await?;
    log::debug!("Rate limit status: {:?}", rate_limit_status.response);

    if rate_limit_status.status_code != 200 {
        let user_ip = client_ip;
        let seconds = rate_limit_status.seconds;
        let mut html_error = File::open("/etc/sheldx/static/rate_limit.html").unwrap();
        let mut html_content = String::new();
        html_error.read_to_string(&mut html_content).unwrap();

        let final_html = html_content
            .replace("{{user_ip}}", &user_ip)
            .replace("{{seconds}}", &seconds.to_string());

        log::debug!("Rate limit response: {:?}", final_html);

        return Ok(
            Response::builder()
                .status(rate_limit_status.status_code)
                .body(Full::from(Bytes::from(final_html)))
                .unwrap()
        );
    }


    // if tls is enabled, return error with this site  is not secure , if you are the owner of this website, please configure it properly or if you are a visitor, please try again later

    // Extract the host from the request

   

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
    let forwarding_rules: Vec<crate::utils::ForwardingRule> = forwarding_rules.ok_or(ProxyError::NoForwardingRules)?;

    log::debug!("Forwarding Rules: {:?}", forwarding_rules);

    println!("{:?}", forwarding_rules);
    // Get the forwarding rule for the host
    let rule = get_forwarding_rule(&Some(forwarding_rules), &host);
 
   if rule.is_err() {
        return Ok(create_error_response(404, "Sorry the page you are looking for is not found", "Page not found"));
    }

    // Connect to the destination server with a timeout
    let destination = rule.unwrap().target  ;
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
   
    http_error_response(
        response.status_code,
       message.to_string(),
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
// pub fn show_html_page(title: &str, message: &str) -> Response<Full<Bytes>> {
//     let html = format!(
//         "<!DOCTYPE html>
//         <html>
//             <head>
//                 <title>{}</title>
//             </head>
//             <body>
//                 <h1>{}</h1>
//                 <p>{}</p>
//             </body>
//         </html>",
//         title,
//         title,
//         message
//     );
//     Response::builder()
//         .status(404)
//         .body(Full::from(Bytes::from(html)))
//         .unwrap()
// }

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
