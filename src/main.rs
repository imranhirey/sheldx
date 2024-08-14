use std::error::Error;

use server::start_sheildx;
use utils::{init_logger, start_redis};
mod handlers;
mod utils;
mod server;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logger()?;  // Ensure logging is initialized


  
        let is_running = start_redis();
        match is_running {
            Ok(_) => {
                println!("Redis server started successfully.");
                start_sheildx().await?;
            }
            Err(e) => {
                println!("Error starting Redis server: {:?}", e);
                log::error!("Error starting Redis server: {:?}", e);
            }
        }
  
    // Graceful shutdown setup

    Ok(())
}
