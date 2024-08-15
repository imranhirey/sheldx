use std::error::Error;

use server::start_sheildx;
use utils:: init_logger ;
mod handlers;
mod utils;
mod server;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logger()?;

    start_sheildx().await.map_err(|e| {
        // check if the is related tp permission denied
        if e.to_string().contains("Permission denied") {
           // tell user to give sheldx permission to bind to port 80 by telling how to do it 
              log::error!("Permission denied. You may need to run Sheldx as root or give it permission to bind to port 80");
              log::info!("You can give Sheldx permission to bind to port 80 by running the following command:");
                log::info!("sudo setcap cap_net_bind_service=+ep /path/to/sheldx");
                
        } else {
            log::error!("Error starting Sheldx: {}", e);
        }


        e
    })?;

    // TODO: Implement graceful shutdown setup

    Ok(())
}
