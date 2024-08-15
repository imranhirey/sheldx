
use hyper::{body::Incoming, Request};

use super::ForwardingRule;

pub fn extract_host(req: &Request<Incoming>) -> Result<String, String> {
    match req.headers().get("host") {
        Some(host) => match host.to_str() {
            Ok(host_str) => Ok(host_str.to_owned()),
            Err(_) => Err("we can not process the request because the host header is not a valid".to_owned()),
        },
        None => Ok("skillhob.com".to_owned()), // Use default value
    }
}


// get forward rule

pub fn get_forwarding_rule(
    rules:& Option<Vec<ForwardingRule>>,
    host: &str
) -> Result<ForwardingRule, String> {
    match rules {
        Some(rules) => {
            for rule in rules {
                if rule.host == host {
                    return Ok(rule.clone());
                }
            }
            Err("The requested URL was not found on this server".to_owned())
        }
        None => Err("The requested URL was not found on this server".to_owned())
    }
   
}