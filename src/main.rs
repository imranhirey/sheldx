// use std::error::Error;

// use server::start_sheildx;
// use utils::init_logger;
// mod utils;
// mod server;
// mod handlers;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     init_logger()?;
//     start_sheildx().await?;

//     Ok(())
// }

use std::net::SocketAddr;



#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::serve::bind(&addr)
        .serve(|| async {
            axum::Router::new()
                .route("/", axum::handler::get(|| async { "Hello, World!" }))
        })
        .await
        .unwrap();

}