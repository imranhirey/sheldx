## SheldX: Your Go-To Proxy for Rate Limiting and More

SheldX is a lightweight and performant proxy server written in Go. It's designed to be easy to use and configure, making it the perfect solution for:

* **Rate limiting** traffic to your applications, preventing abuse and ensuring availability.
* **Forwarding** traffic to different backend servers based on hostnames.
* **Serving static files**, acting as a simple web server.
* **Gaining insights** into your traffic patterns with detailed logging.

**SheldX is currently under active development.** We are constantly adding new features and improvements.

### Key Features

* **Powerful Rate Limiting:**
* Define flexible rate limits based on host, path, and IP address.
* Choose from multiple rate limiting strategies like fixed window, sliding window, and token bucket (using HashMap or Redis).
* Customize limits (requests per duration) and burst capacity.
* Easily exclude specific paths or IP addresses from rate limiting.
* **Simple Forwarding Rules:**
* Forward incoming requests to different target servers based on the requested host.
* **Static File Serving:**
* Serve static content like HTML, CSS, and JavaScript files from a specified directory.
* **TLS Support:**
* Secure communication with both clients and backend servers using TLS encryption (configurable).
* **Detailed Logging:**
* Get insights into your traffic with configurable logging options, including console output.
* **Easy Configuration:**
* Configure SheldX through a simple and intuitive TOML configuration file.
* **Lightweight and Performant:**
* Built with Go for efficiency and speed, ensuring minimal overhead for your applications.

### Getting Started

1. **Clone the repository:**
```bash
git clone https://github.com/imranhirey/sheldx.git
cd sheldx
```

2. **Build SheldX:**
```bash
go build
```

3. **Configure SheldX:**
Open the `config.toml` file and customize the following sections:

```toml
cert_path = "" # Path to your TLS certificate (leave empty if TLS is disabled)
key_path = "" # Path to your TLS private key (leave empty if TLS is disabled)
is_tls_enabled = false
show_logs_on_console = true
static_files_directory = "/etc/sheldx/static/index.html"

[[rate_limit_rules]]
host = "app.localhost:3001" # Target host for rate limiting
limit = 10 # Requests allowed per duration
duration = 60 # Duration in seconds
max_tokens = 1000 # Maximum burst capacity
excluded_paths = ["/health"] # Paths excluded from rate limiting
excluded_ip_list = ["192.168.1.1"] # IPs excluded from rate limiting
strategy = "HashMap" # Rate limiting strategy: "HashMap" or "Redis"

[[rate_limit_rules]]
# ... Add more rate limit rules as needed ...

[[forwarding_rules]]
host = "app.localhost:3001" # Host to match for forwarding
target = "192.168.0.53:3000" # Target server to forward requests to

[[forwarding_rules]]
# ... Add more forwarding rules as needed ...
```

4. **Run SheldX:**
```bash
./sheldx
```

### Examples

**Rate Limit Example (using Redis):**

```toml
[[rate_limit_rules]]
host = "api.example.com"
limit = 100
duration = 60
max_tokens = 200
excluded_paths = ["/login"]
excluded_ip_list = ["10.0.0.1", "172.16.0.0/12"]
strategy = "Redis"
```

This configuration limits requests to `api.example.com` to 100 requests per minute, with a burst capacity of 200 requests. Requests to `/login`, and requests from IPs within the specified ranges, are excluded from rate limiting.


**Forwarding Example:**

```toml
[[forwarding_rules]]
host = "app1.example.com"
target = "192.168.1.10:8080"

[[forwarding_rules]]
host = "app2.example.com"
target = "192.168.1.20:8080"
```

This configuration forwards requests for `app1.example.com` to a server at `192.168.1.10:8080` and requests for `app2.example.com` to a different server at `192.168.1.20:8080`.


### Contributing

We welcome contributions to SheldX! If you'd like to contribute, please open an issue or submit a pull request on GitHub.

### License

SheldX is open-source software licensed under the [MIT License](https://opensource.org/licenses/MIT).