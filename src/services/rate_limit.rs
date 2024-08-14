/*
i am usiing toml  for my configs 

[[rate_limit]]
enabled = true // first look for this
limit = 100
duration = 60
exclude = ["/health"] // second look for this
// we have two stregies for ratelimiting local hash map and redis so user can choose which one to use

strategy = "hashmap" // third look for this

*/


pub fn enforce_rate_limit(
    req: &Request<hyper::body::Body>,
    client_ip: &str,
    rate_limiter_map: &RateLimiterMap,
) -> Result<(), ProxyError> {
    let rate_limiters = rate_limiter_map.lock().unwrap();
    let rate_limiter = rate_limiters
        .get(client_ip)
        .unwrap_or_else(|| {
            let configs = load_configs().map_err(|_| ProxyError::ConfigError)?;
            let rate_limit = configs.rate_limit.iter().find(|rate_limit| {
                rate_limit.exclude.iter().all(|path| !req.uri().path().starts_with(path))
            });

            match rate_limit {
                Some(rate_limit) => {
                    let rate_limiter = Ratelimiter::builder(rate_limit.limit, Duration::from_secs(rate_limit.duration))
                        .max_tokens(rate_limit.limit)
                        .build()
                        .unwrap();
                    rate_limiter
                }
                None => {
                    let rate_limiter = Ratelimiter::builder(10, Duration::from_secs(60))
                        .max_tokens(10)
                        .build()
                        .unwrap();
                    rate_limiter
                }
            }
        });

    // Apply rate limiting
    if let Err(sleep) = rate_limiter.try_wait() {
        // Rate limit exceeded

        let title = "<h1>429 Too Many Requests</h1>";
        let message = format!(
            "<p>Rate limit exceeded. Try again in {} seconds</p>",
            sleep.as_secs()
        );
        let response = show_html_page(title, &message);
        return Err(ProxyError::HttpCommError);
    }

    Ok(())
}