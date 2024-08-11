// use std::{ convert::Infallible, fs };

// use http_body_util::{ BodyExt, Full };
// use hyper::{
//     body::{ Body, Bytes, Incoming },
//     client::conn::http1::{ handshake, Connection, SendRequest },
//     Request,
//     Response,
// };
// use hyper_util::rt::TokioIo;
// use tokio::net::TcpStream;

// use crate::utils::load_configs;

// pub async fn http_proxy(
//     rreq: Request<hyper::body::Incoming>
// ) -> Result<Response<Full<Bytes>>, Infallible> {
//     let configs = load_configs().unwrap();

//     if configs.forwarding_rules.is_none() {
//         if configs.static_files_directory.is_none() {
//             // return error
//             let home_dir = dirs::home_dir().ok_or("Could not find home directory").unwrap();
//             //"C:\Users\imranhirey\sheldx\defaults\static\index.html"
//             let err_html =
//                 "<html><head><title>404 Not Found</title></head><body><h1>404 Not Found</h1><p>The requested URL was not found on this server.</p></body></html>";
//             let default_page = fs
//                 ::read_to_string(
//                     home_dir.to_str().unwrap_or(err_html).to_string() +
//                         "/sheldx/defaults/static/index.html"
//                 )
//                 .unwrap_or(err_html.to_string());

//             return Ok(
//                 Response::builder()
//                     .status(404)
//                     .body(Full::from(Bytes::from(default_page)))
//                     .unwrap()
//             );
//         } else {
//             let file = fs::read_to_string(configs.static_files_directory.unwrap()).unwrap();
//             return Ok(
//                 Response::builder()
//                     .status(404)
//                     .body(Full::from(Bytes::from(file)))
//                     .unwrap()
//             );
//         }
//     } else {
//         let fowarding_rules = configs.forwarding_rules.unwrap();
//         println!("{:?}", fowarding_rules);
//         let host = rreq.headers().get("host").unwrap().to_str().unwrap();
//         for rule in fowarding_rules {
//             println!("Forwarding rule found {:?}", host);

//             if rule.host == host {
//                 let destination = rule.destination;
//                 let stream = tokio::net::TcpStream::connect(destination.clone()).await.unwrap();
//                 println!("Connected to server");
//                 // prinnt the stream
//                 println!("{:?}", stream);
//                 let io = TokioIo::new(stream);
//                 let (mut send_request, connection): (
//                     SendRequest<Incoming>,
//                     Connection<TokioIo<TcpStream>, Incoming>,
//                 ) = handshake(io).await.unwrap();

//                 tokio::task::spawn(async move {
//                     if let Err(err) = connection.await {
//                         eprintln!("Error serving connection: {:?}", err);
//                         log::error!("Error serving connection: {:?}", err);
//                     }
//                 });

//                 let simple_body = Bytes::from("Hello, World!");
//                 // create a request
//                     // convert the buffer to bytes
 
//     // convert to full
//     let _bodyfull: Full<Bytes> = Full::from(simple_body);


//                 let (parts, body) = rreq.into_parts();
//                 let request = Request::from_parts(parts, _bodyfull);

//                 let mut res = send_request.send_request(rreq).await.unwrap();
//                 let mut buff = Vec::new();
//                 while let Some(next) = res.frame().await {
//                     let frame = next.unwrap();
//                     if let Some(chunk) = frame.data_ref() {
//                         buff.extend_from_slice(chunk);
//                     }
//                 }

//                 println!("Response received from server express u eke {:?}", buff);

//                 // convert the buffer to bytes
//                 let bytes = Bytes::from(buff);
//                 // convert to full
//                 let bodyfull: Full<Bytes> = Full::from(bytes);

//                 // GET THE PARTS OF THE RESPONSE
//                 let (mut parts, _body) = res.into_parts();
//                 // the the x-forwarded-for header to  sheldx
//                 parts.headers.insert("x-powered-by", "sheldx Services".parse().unwrap());
//                 // create a response
//                 let response = Response::from_parts(parts, bodyfull);
//                 log::info!("Response received from server");
//                 println!("{:?}", response);
//                 return Ok(response);
//             }
//         }
//     }

//     let stream = tokio::net::TcpStream::connect("192.168.0.36:8000").await.unwrap();

//     let io = TokioIo::new(stream);
//     let (mut send_request, connection): (
//         SendRequest<Incoming>,
//         Connection<TokioIo<TcpStream>, Incoming>,
//     ) = handshake(io).await.unwrap();

//     // send the request to the server

//     tokio::task::spawn(async move {
//         if let Err(err) = connection.await {
//             eprintln!("Error serving connection: {:?}", err);
//             log::error!("Error serving connection: {:?}", err);
//         }
//     });

//     let mut res = send_request.send_request(rreq).await.unwrap();
//     log::info!("Request sent to server");

//     let mut buff = Vec::new();
//     while let Some(next) = res.frame().await {
//         let frame = next.unwrap();
//         if let Some(chunk) = frame.data_ref() {
//             buff.extend_from_slice(chunk);
//         }
//     }

//     // convert the buffer to bytes
//     let bytes = Bytes::from(buff);
//     // convert to full
//     let bodyfull: Full<Bytes> = Full::from(bytes);

//     // GET THE PARTS OF THE RESPONSE
//     let (mut parts, _body) = res.into_parts();
//     // the the x-forwarded-for header to  sheldx
//     parts.headers.insert("x-powered-by", "sheldx Services".parse().unwrap());
//     // create a response
//     let response = Response::from_parts(parts, bodyfull);
//     log::info!("Response received from server");
//     return Ok(response);

//     //
// }
