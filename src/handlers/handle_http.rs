use std::convert::Infallible;
use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, client::conn::http1, Request, Response};
use crate::utils::{extract_host, get_forwarding_rule, http_error_response, load_configs, HttpMessageError};

pub async fn handle_http_connections(
    req: Request<Incoming>
) -> Result<Response<Full<Bytes>>, Infallible> {
    // Helper function to create an error response
    fn create_error_response(status_code: u16, message: &str, title: &str) -> Result<Response<Full<Bytes>>, Infallible> {
        let response = HttpMessageError {
            status_code,
            message: message.to_string(),
            title: title.to_string(),
        };
        log::error!("{}", message);
        // bounce the error response by nt giving too much information to the client
        let client_message = "we have some internal server error if this persists please contact the admin";
        Ok(http_error_response(response.status_code, client_message.to_string(), response.title).unwrap())
    }

    // Get the configs
    let configs = match load_configs() {
        Ok(configs) => configs,
        Err(_) => return create_error_response(500, "we have some internal server error if this persists please contact the admin", "Internal Server Error"),
    };

    // Get the host from the request
    let host = match extract_host(&req) {
        Ok(host) => host,
        Err(_) => return create_error_response(400, "we can not process the request because the host header is not a valid", "Bad Request"),
    };
  

    // Check if at least one forwarding rule is set
    if configs.forwarding_rules.is_none() {
        return create_error_response(404, "The requested URL was not found on this server", "Not Found");
    }

    // Get the forwarding rules

    let fowarding_rule=get_forwarding_rule(&configs.forwarding_rules, &host);

    let rule = match fowarding_rule {
        Ok(rule) => rule,
        Err(_) => return create_error_response(404, "The requested URL was not found on this server", "Not Found "),
    };


    // Connect to the destination server
    let destination = rule.destination;
    let stream = match tokio::net::TcpStream::connect(destination).await {
        Ok(stream) => stream,
        Err(_) => return create_error_response(500, "Failed to connect to the destination server", "Internal Server Error"),
    };

    // Create a new connection
    let io = hyper_util::rt::TokioIo::new(stream);
    let (mut send_request, connection) = match http1::handshake(io).await {
        Ok((send_request, connection)) => (send_request, connection),
        Err(_) => return create_error_response(500, "Failed to create a connection to the destination server", "Internal Server Error"),
    };

    // Send the request to the destination server
    match send_request.send_request(req).await {
        Ok(_) => (),
        Err(_) => return create_error_response(500, "we have some internal server error if this persists please contact the admin", "Internal Server Error"),
    };
    
    

    Ok(Response::new(Full::new(Bytes::from_static(b"Hello, World!"))))
}