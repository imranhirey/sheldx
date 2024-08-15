use crate::{ utils::load_configs };

use super::{ Server, WithoutTLS };

pub async fn start_sheildx() -> Result<(), Box<dyn std::error::Error>> {
    let configs = load_configs()?;
    // log::debug!("ma ufaran yahay: {:?}", configs.is_tls_enabled);

    if configs.is_tls_enabled {

   
        unimplemented!();
    } else {
        log::warn!("Sheldx recommends using TLS for production use");
        println!("Sheldx recommends using TLS for production use");
        let server = WithoutTLS {};
        server.start().await?;
    }

    Ok(())
}
