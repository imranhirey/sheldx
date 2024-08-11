use std::sync::Arc;

use tokio::sync::RwLock;

use super::{Server, WithoutTLS};

pub struct StartHttpResponse {
    pub is_success: bool,
    pub message: String,
}

pub struct StartHttpError {
    pub message: String,
}

pub async fn start_sheildx()  ->Result<(),Box<dyn std::error::Error>>
 {
let server= WithoutTLS{};

    let res=server.start().await?;

    Ok(())
    
   
    
}
