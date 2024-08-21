use http_body_util::Full;
use hyper::{ body::{ Bytes, Incoming }, Request, Response };

use super::ProxyError;

pub async fn handle_https_connections(
  req: Request<Incoming>
) -> Result<Response<Full<Bytes>>, ProxyError> {

    // setup https connection
  Ok(
    Response::builder()
      .status(200)
      .body(Full::from(Bytes::from("Hello, HTTPS!")))
      .unwrap()
  )
}
