use std::{convert::Infallible, error::Error, fs::File, io::BufReader, net::{IpAddr, Ipv4Addr, SocketAddr}, sync::Arc};
mod server;
mod utils;
mod handlers;
use http_body_util::{BodyExt, Full};
use hyper::{body::{Body, Bytes, Incoming}, server::conn::http1, service::service_fn, Method, Request, Response, StatusCode};
use hyper_util::{rt::{TokioExecutor, TokioIo}, server::conn::auto::Builder};
use rustls::server::NoClientAuth;
use server::start_sheildx;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use utils::init_logger;

#[tokio::main]
async fn main()  -> Result<(),Box<dyn Error>>
 {
    init_logger()?;
    start_sheildx().await?;
    Ok(())
  
   
}





/*
 async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
        Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
    }

    let mut cert_path = "cert.pem".to_string();
    let mut key_path = "key.pem".to_string();

    let mut cert_file = BufReader::new(File::open(cert_path)?); // Use `?` for error handling
    let mut key_file = BufReader::new(File::open(key_path)?);   // Use `?` for error handling

    // Load certificates
    let certs = rustls_pemfile::certs(&mut cert_file)
        .collect::<Result<Vec<_>, _>>()?; // Use `?` for error handling

    let private_key = rustls_pemfile::private_key(&mut key_file)?;  // Use `?` for error handling

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key.unwrap())
        .unwrap();

    let acceptor = TlsAcceptor::from(Arc::new(config.clone()));

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let listener = TcpListener::bind(addr).await?;
let service = service_fn(echo);
    loop {
        let (stream, _) = listener.accept().await?;

        
        let tls_acceptor = acceptor.clone();
        tokio::task::spawn(async move {
            let tls_stream = match tls_acceptor.accept(stream).await {
                Ok(tls_stream) => tls_stream,
                Err(err) => {
                    eprintln!("failed to perform tls handshake: {err:#}");
                    return;
                }
            };
            if let Err(err) = Builder::new(TokioExecutor::new())
                .serve_connection(TokioIo::new(tls_stream), service.clone())
                .await
            {
                eprintln!("failed to serve connection: {err:#}");
            }
        });
        
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
            *response.body_mut() = Full::from(
                req.into_body()
                    .collect()
                    .await?
                    .to_bytes(),
            );
        }
        // Catch-all 404.
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Ok(response) */