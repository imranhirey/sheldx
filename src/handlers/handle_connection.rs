use std::error::Error as StdError;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::utils::Configs;

#[derive(Debug)]
pub struct ForwardingRule {
    pub host: String,
    pub destination: String,
}

#[derive(Debug)]
pub struct ForwardingRuleResponse {
    pub is_found: bool,
    pub rule: Option<ForwardingRule>,
}

pub async fn handle_non_tls_connection(
    mut socket: TcpStream,
    configs: Arc<Configs>
) -> Result<(), Box<dyn StdError + Send + Sync>> {
    // Extract the remote address of the peer
    let peer_addr = socket.peer_addr()?;
    println!("Connected to: {:?}", peer_addr);

    // Extract the hostname from the request headers (assuming HTTP)
    let mut buffer = [0; 1024];
    let _ = socket.readable().await?;
    let n = socket.try_read(&mut buffer)?;
    let request_line = String::from_utf8_lossy(&buffer[..n]);
    let host = request_line
        .lines()
        .find(|line| line.starts_with("Host:"))
        .map(|line| line[5..].trim().to_string());

    match host {
        Some(hostname) => {
            // Match the hostname with forwarding rules
            let forward_rule = get_forward_rule(&configs, &hostname);

            if let Some(rule) = forward_rule {
              log::info!("Forwarding rule found for host: {}", hostname);
                // Forward request logic would go here (not implemented in this example)
            } else {
               log::info!("Forwarding rule not found for host: {}", hostname);

                // Serve static files if no forwarding rule is found
                let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
                let html_path = home_dir.join("sheldx/statics/index.html");
                let html_content = tokio::fs::read(html_path).await?;

                let http_response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    html_content.len(),
                    String::from_utf8_lossy(&html_content)
                );
                socket.write_all(http_response.as_bytes()).await?;
            }
        }
        None => {
            eprintln!("Host header not found in request");
        }
    }

    Ok(())
}

fn get_forward_rule(configs: &Configs, host: &str) -> Option<ForwardingRule> {
    let fowarding_rules = configs.forwarding_rules.as_ref();
    match fowarding_rules {
        Some(rules) => {
            let rule = rules.iter().find(|rule| rule.host == host)?;
            let fowardingrule = ForwardingRule {
                host: rule.host.clone(),
                destination: rule.destination.clone(),
            };
            Some(fowardingrule)
        }
        None => None,
    }
}
