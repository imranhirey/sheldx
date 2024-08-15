
## Sheldx: A Simple and Powerful Proxy with Rate Limiting

[![Go Report Card](https://goreportcard.com/badge/github.com/imranhirey/sheldx)](https://goreportcard.com/report/github.com/imranhirey/sheldx)
[![GitHub license](https://img.shields.io/github/license/imranhirey/sheldx)](https://github.com/imranhirey/sheldx/blob/main/LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/imranhirey/sheldx)](https://github.com/imranhirey/sheldx/releases)

**Sheldx** is a lightweight and efficient proxy server written in Go. It offers simple configuration, flexible routing, and robust rate limiting capabilities to protect your backend services.

**Key Features:**

- **Easy Configuration:** Get started quickly with a human-readable configuration file.
- **Dynamic Forwarding Rules:** Define rules to route traffic to different target servers based on the incoming host.
- **Rate Limiting:** Implement rate limiting using either an in-memory `HashMap` or a `Redis` backend for distributed scenarios.
- **Granular Control:** Configure rate limits per host, path, or IP address.
- **TLS Support:** Secure your proxy with TLS encryption (configurable).
- **Customizable Logging:** Control the level of logging detail displayed on the console.

**Website:** [https://sheldx.com/](https://sheldx.com/)
**GitHub:** [https://github.com/imranhirey/sheldx](https://github.com/imranhirey/sheldx)

## Installation

1. **Download the latest release:** [https://github.com/imranhirey/sheldx/releases](https://github.com/imranhirey/sheldx/releases)

2. **Or build from source:**
```bash
go build
```

## Configuration

Sheldx uses a simple TOML configuration file. Here's an example:

```toml
cert_path = ""
key_path = ""
is_tls_enabled = false
show_logs_on_console = true
static_files_directory = "/etc/sheldx/static/index.html"

[[rate_limit_rules]]
host = "app.localhost:3001"
limit = 10
duration = 60
max_tokens = 1000
excluded_paths = ["/health"]
excluded_ip_list = ["192.168.1.1"]
strategy = "HashMap"

[[rate_limit_rules]]
host = "*"
limit = 5
duration = 60
max_tokens = 500
excluded_paths = ["/status"]
excluded_ip_list = []
strategy = "Redis"

[[forwarding_rules]]
host="app.localhost:3001"
target="192.168.0.53:3000"
```

**Configuration Options:**

- **cert_path:** Path to your TLS certificate file.
- **key_path:** Path to your TLS private key file.
- **is_tls_enabled:** Enable/disable TLS encryption.
- **show_logs_on_console:** Show logs on the console.
- **static_files_directory:** Directory path for serving static files.

**Rate Limit Rules:**

- **host:** Hostname or wildcard pattern to apply the rule.
- **limit:** Number of requests allowed per duration.
- **duration:** Time window in seconds for the rate limit.
- **max_tokens:** Maximum burst capacity for requests.
- **excluded_paths:** List of paths to exclude from rate limiting.
- **excluded_ip_list:** List of IP addresses to exclude from rate limiting.
- **strategy:** Rate limiting strategy (HashMap or Redis).

**Forwarding Rules:**

- **host:** Hostname or wildcard pattern to match incoming requests.
- **target:** Target server address to forward requests to.

## Running Sheldx

1. Save your configuration file (e.g., `sheldx.toml`).
2. Run Sheldx:

```bash
./sheldx -config sheldx.toml
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or feature request
