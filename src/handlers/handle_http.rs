use std::convert::Infallible;
use http_body_util::{ BodyExt, Full };
use hyper::{ body::{ Buf, Bytes, Incoming }, client::conn::http1, Request, Response };
use tokio::spawn;
use crate::utils::{
    extract_host,
    get_forwarding_rule,
    http_error_response,
    load_configs,
    HttpMessageError,
};

pub async fn handle_http_connections(
    req: Request<Incoming>
) -> Result<Response<Full<Bytes>>, Infallible> {
    // Helper function to create an error response
    fn create_error_response(
        status_code: u16,
        message: &str,
        title: &str
    ) -> Result<Response<Full<Bytes>>, Infallible> {
        let response = HttpMessageError {
            status_code,
            message: message.to_string(),
            title: title.to_string(),
        };
        log::error!("{}", message);
        // bounce the error response by nt giving too much information to the client
        let client_message =
            "we have some internal server error if this persists please contact the admin";
        Ok(
            http_error_response(
                response.status_code,
                client_message.to_string(),
                response.title
            ).unwrap()
        )
    }

    // Get the configs
    let configs = match load_configs() {
        Ok(configs) => configs,
        Err(_) => {
            return create_error_response(
                500,
                "we have some internal server error if this persists please contact the admin",
                "Internal Server Error"
            );
        }
    };

    log::debug!("Configs: {:?}", configs);
    // Get the host from the request
    let host = match extract_host(&req) {
        Ok(host) => host,
        Err(_) => {
            return create_error_response(
                400,
                "we can not process the request because the host header is not a valid",
                "Bad Request"
            );
        }
    };
    log::debug!("Host: {:?}", host);

    // Check if at least one forwarding rule is set
    if configs.forwarding_rules.is_none() {
        return create_error_response(
            404,
            "The requested URL was not found on this server",
            "Not Found"
        );
    }

    // Get the forwarding rules

    let fowarding_rule = get_forwarding_rule(&configs.forwarding_rules, &host);
    log::debug!("Forwarding Rule: {:?}", fowarding_rule);

    let rule = match fowarding_rule {
        Ok(rule) => rule,
        Err(_) => {
            return create_error_response(
                404,
                "The requested URL was not found on this server",
                "Not Found "
            );
        }
    };

    // Connect to the destination server
    let destination = rule.destination;
    log::debug!("Destination: {:?}", destination);
    let stream = match tokio::net::TcpStream::connect(destination).await {
        Ok(stream) => stream,
        Err(e) => {
            log::error!("Failed to connect to the destination server: {:?}", e);
            return create_error_response(
                500,
                "Failed to connect to the destination server",
                "Internal Server Error"
            );
        }
    };

    // Create a new connection
    let io = hyper_util::rt::TokioIo::new(stream);
    let (mut send_request, connection) = match http1::handshake(io).await {
        Ok((send_request, connection)) => (send_request, connection),
        Err(_) => {
            return create_error_response(
                500,
                "Failed to create a connection to the destination server",
                "Internal Server Error"
            );
        }
    };

    tokio::task::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("Error serving connection: {:?}", err);
            log::error!("Error serving connection: {:?}", err);
        }
    });

    // Send the request to the destination server
    let res = send_request.send_request(req).await.unwrap();

    let (parts, body) = res.into_parts();

    // convert the body to bytes
    let bytes = match body.collect().await {
        Ok(bytes) => bytes,
        Err(_) => {
            return create_error_response(
                500,
                "Failed to read the response body",
                "Internal Server Error"
            );
        }
    };

    let finalbod: Full<Bytes> = Full::from(bytes.to_bytes());

    let response = Response::from_parts(parts, finalbod);
    Ok(response)
}
