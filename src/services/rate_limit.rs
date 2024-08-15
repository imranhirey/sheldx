/*
Configuration using TOML for rate limiting.

[[rate_limit]]
enabled = true  # Enable rate limiting
limit = 100     # Number of requests allowed
duration = 60   # Time period in seconds for the limit
exclude = ["/health"]  # Paths to exclude from rate limiting
strategy = "hashmap"  # Choose between "hashmap" or "redis" for the rate-limiting strategy
*/

use std::time::Duration;
use hyper::{ Request };
use ratelimit::Ratelimiter;
use crate::{
    handlers::{ ProxyError },
    server::RateLimiterMap,
    utils::{extract_host, Configs},
};

pub struct RateLimitResponse {
    pub response: String,
    pub status_code: u16,
    pub seconds: u64,
}
pub async fn enforce_rate_limit(
    req: &Request<hyper::body::Incoming>,
    client_ip: &str,
    rate_limiter_map: &RateLimiterMap,
    config: &Configs
) -> Result<RateLimitResponse, ProxyError> {
    log::debug!("Client IP: {:?}", client_ip);

    let host = match extract_host(req) {
        Ok(host) => host,
        Err(_) => return Err(ProxyError::HostError),
    };
    let path = req.uri().path();

    // Check if there are rate limit rules
    if let Some(rate_limit_rules) = &config.rate_limit_rules {
        // Try to find a rule that matches the host
        let rule = rate_limit_rules.iter()
            .find(|rule| rule.host == host)
            // If no specific rule matches, fall back to the wildcard rule
            .or_else(|| rate_limit_rules.iter().find(|rule| rule.host == "*"));

        if let Some(rule) = rule {
            // Check if the request path is in the exclude list
            if rule.excluded_paths.iter().any(|p| path.starts_with(p)) {
                return Ok(RateLimitResponse {
                    response: String::new(),
                    status_code: 200,
                    seconds: 0,
                });
            }

            // Check if the IP is in the excluded list
            if rule.excluded_ip_list.contains(&client_ip.to_string()) {
                return Ok(RateLimitResponse {
                    response: String::new(),
                    status_code: 200,
                    seconds: 0,
                });
            }

            // Enforce the rate limit based on the rule's settings
            let mut rate_limiters = rate_limiter_map.lock().await;
            let rate_limiter = rate_limiters
                .entry(client_ip.to_string())
                .or_insert_with(|| {
                    Ratelimiter::builder(rule.limit, Duration::from_secs(rule.duration))
                        .max_tokens(rule.max_tokens)
                        .initial_available(rule.limit)
                        .build()
                        .unwrap()
                });

            match rate_limiter.try_wait() {
                Ok(()) => Ok(RateLimitResponse {
                    response: String::new(),
                    status_code: 200,
                    seconds: 0,
                }),
                Err(seconds) => Ok(RateLimitResponse {
                    response: format!(
                        "Rate limit exceeded. Try again in {} seconds",
                        seconds.as_secs()
                    ),
                    status_code: 429,
                    seconds: seconds.as_secs(),
                }),
            }
        } else {
            // No matching rule found, apply default behavior if needed
            log::info!("No rate limit rule found for host: {} so no rate limit applied", host);
            Ok(RateLimitResponse {
                response: String::new(),
                status_code: 200,
                seconds: 0,
            })
        }
    } else {
        // No rate limit rules specified, apply default behavior if needed
        Ok(RateLimitResponse {
            response: String::new(),
            status_code: 200,
            seconds: 0,
        })
    }
}
