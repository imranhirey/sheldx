package config

import (
	"fmt"

	"github.com/BurntSushi/toml"
)

type Config struct {
	CertPath             string          `toml:"cert_path"`
	KeyPath              string          `toml:"key_path"`
	IsTLSEnabled         bool            `toml:"is_tls_enabled"`
	ConnectionTimeout    int             `toml:"connection_timeout"`
	MaxRetries           int             `toml:"max_retries"`
	ShowLogsOnConsole    bool            `toml:"show_logs_on_console"`
	StaticFilesDirectory string          `toml:"static_files_directory"`
	ForwardingRules      []FowardingRule `toml:"forwarding_rules"`
	RateLimitRules       []RateLimitRule `toml:"rate_limit_rules"`
}

type RateLimitRule struct {
	Host           string   `toml:"host"`
	Limit          int      `toml:"limit"`
	Duration       int      `toml:"duration"`
	MaxTokens      int      `toml:"max_tokens"`
	ExcludedPaths  []string `toml:"excluded_paths"`
	ExcludedIPList []string `toml:"excluded_ip_list"`
	Strategy       string   `toml:"strategy"`
}

type FowardingRule struct {
	Host   string `toml:"host"`
	Target string `toml:"target"`
}

func LoadConfig() (*Config, error) {
	var config Config
	if _, err := toml.DecodeFile("/etc/sheldx/configs/main.conf", &config); err != nil {
		return nil, fmt.Errorf("error loading config file: %w", err)
	}
	return &config, nil
}
