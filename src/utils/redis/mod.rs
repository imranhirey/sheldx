use std::error::Error;
use std::process::{Command, Stdio};

pub fn start_redis() -> Result<(), Box<dyn Error>> {
    log::info!("Starting Redis server");

    let child = Command::new("./redis/redis-server")
        .arg("./redis/redis.conf")
        .stdout(Stdio::inherit()) // Inherit stdout to see Redis logs in your console
        .stderr(Stdio::inherit()) // Inherit stderr for error messages
        .spawn();  // Don't wait for the command to complete, just start it

    match child {
        Ok(_) => {
            log::info!("Redis server started successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("Failed to start Redis server: {:?}", e);
            Err(format!("Failed to start Redis server: {:?}", e).into())
        }
    }
}