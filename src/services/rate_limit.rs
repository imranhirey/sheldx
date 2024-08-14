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

// pub fn load_rate_limit_config() -> RateLimitConfig {
//     let rate_limit_config = RateLimitConfig {
//         limit: 100,
//         duration: Duration::from_secs(60),
//     };
//     rate_limit_config
// }
