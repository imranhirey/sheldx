use serde::{ Deserialize, Serialize };
use thiserror::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file read error")]
    ConfigFileReadError,
    #[error(
        "Configuration file parse error this may be due to invalid TOML syntax or invalid configuration"
    )]
    ConfigFileParseError,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ForwardingRule {
    pub host: String,
    pub target: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum RateLimitStrategy {
    HashMap,
    Redis,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RateLimitRule {
    pub host: String,
    pub limit: u64,
    pub duration: u64,
    pub max_tokens: u64,
    pub excluded_paths: Vec<String>,
    pub excluded_ip_list: Vec<String>,
    pub strategy: RateLimitStrategy,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Configs {
    pub cert_path: String,
    pub key_path: String,
    pub is_tls_enabled: bool,
    pub show_logs_on_console: bool,
    pub forwarding_rules: Option<Vec<ForwardingRule>>,
    pub static_files_directory: Option<String>,
    pub rate_limit_rules: Option<Vec<RateLimitRule>>, // Updated to support multiple rules
}

// Check configurations
impl Configs {
    pub fn _check(&self) -> Result<(), ConfigError> {
        if !PathBuf::from(&self.cert_path).exists() {
            log::error!(
                "Certificate file not found at {:?}. Please see how to set up a TLS certificate at https://docs.sheldx.io/docs/setup-tls-certificate",
                self.cert_path
            );
        }

        if !PathBuf::from(&self.key_path).exists() {
            log::error!(
                "Key file not found at {:?}. Please see how to set up a TLS certificate at https://docs.sheldx.io/docs/setup-tls-certificate",
                self.key_path
            );
        }

        if self.is_tls_enabled && (self.cert_path.is_empty() || self.key_path.is_empty()) {
            log::error!("TLS is enabled but certificate or key path is not provided");
        }

        Ok(())
    }
}

pub fn load_configs() -> Result<Configs, ConfigError> {
    let config_dir = PathBuf::from("/etc/sheldx/configs");
    let config_path = config_dir.join("main.conf");

    log::debug!("Trying to load config file from {:?}", config_path);
    log::info!("Loading updated configuration file from {:?}", config_path);

    // Check if configuration file exists
    if !config_path.exists() {
        // Create default config file
        log::warn!("Configuration file not found, creating default configuration file");
        create_default_config().map_err(|_| ConfigError::ConfigFileReadError)?;
    }

    log::info!("Configuration file found at {:?}", config_path);

    // Read configuration file
    let config_string = fs
        ::read_to_string(&config_path)
        .map_err(|_| ConfigError::ConfigFileReadError)?;
    log::debug!("Configuration file updated: {:?}", config_string);
    let config: Result<Configs, toml::de::Error> = toml::from_str(&config_string);

    match config {
        Ok(config) => Ok(config),
        Err(e) => {
            log::error!("Failed to parse configuration file: {:?}", e);
            Err(ConfigError::ConfigFileParseError)
        }
    }
}

pub fn create_default_config() -> Result<(), ConfigError> {
    let config_dir = PathBuf::from("/etc/sheldx/configs");
    let config_path = config_dir.join("main.conf");

    // Ensure the path exists
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|_| ConfigError::ConfigFileReadError)?;
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
            rate_limit_rules: Some(
                vec![
                    RateLimitRule {
                        host: "api.example.com".to_string(),
                        limit: 10,
                        duration: 60,
                        max_tokens: 1000,
                        excluded_paths: vec!["/health".to_string()],
                        excluded_ip_list: vec!["192.168.1.1".to_string()],
                        strategy: RateLimitStrategy::HashMap,
                    },
                    RateLimitRule {
                        host: "public.example.com".to_string(),
                        limit: 5,
                        duration: 60,
                        max_tokens: 500,
                        excluded_paths: vec!["/status".to_string()],
                        excluded_ip_list: vec![],
                        strategy: RateLimitStrategy::Redis,
                    }
                ]
            ),
        };

        let default_config_string = toml
            ::to_string(&default_config)
            .map_err(|_| ConfigError::ConfigFileParseError)?;
        fs
            ::write(&config_path, default_config_string)
            .map_err(|_| ConfigError::ConfigFileReadError)?;

        // Check if static files directory exists
        log::info!("Creating default static files directory at /etc/sheldx/static");
        let static_files_dir = PathBuf::from("/etc/sheldx/static");
        if !static_files_dir.exists() {
            fs::create_dir_all(&static_files_dir).map_err(|_| ConfigError::ConfigFileReadError)?;
            fs
                ::write(static_files_dir.join("index.html"), "<h1>Welcome to Sheldx</h1>")
                .map_err(|_| ConfigError::ConfigFileReadError)?;
        }

        log::info!("Default configuration file created at {:?}", config_path);
    }

    Ok(())
}
