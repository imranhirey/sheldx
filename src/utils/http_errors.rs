use std::convert::Infallible;

use http_body_util::Full;
use hyper::{ body::Bytes, Response };

pub struct HttpMessageError {
    pub status_code: u16,
    pub message: String,
    pub title: String,
}

pub fn http_error_response(
    status_code: u16,
    message: String,
    title: String
) -> Result<Response<Full<Bytes>>, Infallible> {
    let error = HttpMessageError {
        status_code,
        message: message.to_string(),
        title: title.to_string(),
    };

    // create simple html response with the error message and status code and title and make it center of the page

    let http = format!(
        r#"<html>
        <head>
            <title>{title}</title>
        </head>
        <body>
            <div style="display: flex; justify-content: center; align-items: center; height: 100vh;">
                <div style="text-align: center;">
                    <h1>{status_code}</h1>
                    <p>{message}</p>
                </div>
            </div>
        </body>
    </html>"#,
        title = error.title,
        status_code = error.status_code,
        message = error.message
    );

    let response = Response::builder()
        .status(status_code)
        .header("Content-Type", "text/html")
        .body(Full::from(Bytes::from(http)))
        .unwrap();

    Ok(response)
}
