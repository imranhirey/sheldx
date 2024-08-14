use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::error::Error;



#[derive(Debug, Deserialize, Clone, Serialize)]

pub enum RateLimitStrategy {
    HashMap,
    Redis,
}
#[derive(Debug, Deserialize, Clone, Serialize)]

pub struct RateLimitConfig {
    pub limit: u32,
    pub duration: u64,
    pub exclude: Vec<String>,
    pub strategy: RateLimitStrategy,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ForwardingRule {
    pub host: String,
    pub destination: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Configs {
    pub cert_path: String,
    pub key_path: String,
    pub is_tls_enabled: bool,
    pub show_logs_on_console: bool,
    pub forwarding_rules: Option<Vec<ForwardingRule>>,
    pub static_files_directory: Option<String>,
    pub rate_limit: Option<RateLimitConfig>,
}

// check configurations
impl Configs {
    pub fn _check(&self) -> Result<(), Box<dyn Error>> {
        if !PathBuf::from(&self.cert_path).exists() {
            log::error!(
                "Certificate file not found at {:?} please see how to setup a tls certificate at https://docs.sheldx.io/docs/setup-tls-certificate",
                self.cert_path
            );
        }

        if !PathBuf::from(&self.key_path).exists() {
            return Err(format!("Key file not found at {:?}", self.key_path).into());
        }

        if self.is_tls_enabled && (self.cert_path.is_empty() || self.key_path.is_empty()) {
            return Err("Certificate and key paths must be provided when TLS is enabled".into());
        }

        Ok(())
    }
}

pub fn load_configs() -> Result<Configs, Box<dyn Error>> {
    let config_dir = PathBuf::from("/etc/sheldx/configs");
    let config_path = config_dir.join("main.conf");
    log::info!("Loading configuration file from {:?}", config_path);

    // Check if configuration file exists
    
        // Create default config file
        create_default_config()?;
    
    log::info!("Configuration file found at {:?}", config_path);

    // Read configuration file
    let config_string = fs::read_to_string(&config_path)?;
    let config: Result<Configs, toml::de::Error> = toml::from_str(&config_string);

    match config {
        Ok(config) => Ok(config),
        Err(e) => {
            log::error!("Failed to parse configuration file: {:?}", e);
            Err(format!("Failed to parse configuration file: {:?}", e).into())
        }
    }
}

pub fn create_default_config() -> Result<(), Box<dyn Error>> {
    let config_dir = PathBuf::from("/etc/sheldx/configs");
    let config_path = config_dir.join("config.toml");

    // Ensure the path exists
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    // Create default configuration
    if !config_path.exists() {
        let default_config = Configs {
            cert_path: String::from(""),
            key_path: String::from(""),
            is_tls_enabled: false,
            show_logs_on_console: true,
            forwarding_rules: None,
            static_files_directory: Some(String::from("/etc/sheldx/static/index.html")),
            rate_limit:
                Some(RateLimitConfig {
                    limit: 100,
                    duration: 60,
                    exclude: vec![String::from("/health")],
                    strategy: RateLimitStrategy::HashMap,
                }),
        };

        let default_config_string = toml::to_string(&default_config)?;
        fs::write(&config_path, default_config_string)?;

        // check if static files directory exists
        log::info!("Creating default static files directory at /etc/sheldx/static");
        let static_files_dir = PathBuf::from("/etc/sheldx/static");
        if !static_files_dir.exists() {
            fs::create_dir_all(&static_files_dir)?;
            fs::write(static_files_dir.join("index.html"), "<h1>Welcome to Sheldx</h1>")?;
        }

        log::info!("Default configuration file created at {:?}", config_path);


    }

    Ok(())
}
