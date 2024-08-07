use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct ForwardingRule {
    pub host: String,
    pub destination: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configs {
    pub cert_path: String,
    pub key_path: String,
    pub is_tls_enabled: bool,
    pub show_logs_on_console: bool,
    pub forwarding_rules: Option<Vec<ForwardingRule>>,
    pub static_files_directory: Option<String>,
}

// chec configarations

impl Configs {
    pub fn _check(&self) -> Result<(), Box<dyn Error>> {
        if !PathBuf::from(&self.cert_path).exists() {
            // return Err(format!("Certificate file not found at {:?} please see how to setup a tls certificate at https://docs.sheldx.io/docs/setup-tls-certificate", self.cert_path).into());

            log::error!(
                "Certificate file not found at {:?} please see how to setup a tls certificate at https://docs.sheldx.io/docs/setup-tls-certificate",
                self.cert_path
            );
        }

        if !PathBuf::from(&self.key_path).exists() {
            return Err(format!("Key file not found at {:?}", self.key_path).into());
        }
        // if tls is enabled but the certificate and key paths are not provided
        if self.is_tls_enabled && (self.cert_path.is_empty() || self.key_path.is_empty()) {
            return Err("Certificate and key paths must be provided when TLS is enabled".into());
        }

        Ok(())
    }
}

pub fn load_configs() -> Result<Configs, Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let config_path: PathBuf = [
        home_dir.to_str().ok_or("Invalid home directory string")?,
        "sheldx",
        "configs",
        "config.toml",
    ]
        .iter()
        .collect();

    // Ensure the path is valid
    if !config_path.exists() {
        return Err(format!("Configuration file not found at {:?}", config_path).into());
    }

    let config_string = fs::read_to_string(&config_path)?;
    let _config: Result<Configs, toml::de::Error> = toml::from_str(&config_string);

    let config = match _config {
        Ok(config) => config,
        Err(e) => {
            log::error!("Failed to parse configuration file: {:?}", e);
            return Err(format!("Failed to parse configuration file: {:?}", e).into());
        }
    };

    Ok(config)
}
