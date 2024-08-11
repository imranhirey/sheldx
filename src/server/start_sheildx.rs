use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{server::WithTls, utils::load_configs};

use super::{ Server, WithoutTLS };

pub struct StartHttpResponse {
    pub is_success: bool,
    pub message: String,
}

pub struct StartHttpError {
    pub message: String,
}

pub async fn start_sheildx() -> Result<(), Box<dyn std::error::Error>> {
    let configs = load_configs()?;
    // log::debug!("ma ufaran yahay: {:?}", configs.is_tls_enabled);
    println!("ma uf aran yahay: {:?}", configs.is_tls_enabled);

    if configs.is_tls_enabled {
      let server =WithTls{};
        server.start().await?;
    } else {
        let server = WithoutTLS {};
        server.start().await?;
    }

    Ok(())
}
