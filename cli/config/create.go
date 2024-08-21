package config

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/BurntSushi/toml"
)

// CreateConfigFile creates or overwrites the config file with default values
func CreateConfigFile() error {
	config := Config{
		CertPath:             "/etc/sheldx/certs/cert.pem",
		KeyPath:              "/etc/sheldx/certs/key.pem",
		IsTLSEnabled:         false,
		ConnectionTimeout:    10,
		MaxRetries:           3,
		ShowLogsOnConsole:    true,
		StaticFilesDirectory: "/etc/sheldx/static/index.html",
		RateLimitRules: []RateLimitRule{
			{
				Host:           "example.com",
				Limit:          10,
				Duration:       60,
				MaxTokens:      10,
				ExcludedPaths:  []string{"/health"},
				ExcludedIPList: []string{},
				Strategy:       "local",
			},
		},
	}

	// Define the directory and file path
	configDir := "/etc/sheldx/configs"
	configFilePath := filepath.Join(configDir, "main.conf")

	// Check if the directory exists, create it if not
	if _, err := os.Stat(configDir); os.IsNotExist(err) {
		err := os.MkdirAll(configDir, 0755)
		if err != nil {
			return fmt.Errorf("failed to create config directory: %v", err)
		}
	}

	// Open the file for writing (create if it doesn't exist, truncate if it does)
	file, err := os.OpenFile(configFilePath, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)
	if err != nil {
		return fmt.Errorf("failed to open config file: %v", err)
	}
	defer file.Close()

	// Encode the config struct into the file in TOML format
	if err := toml.NewEncoder(file).Encode(config); err != nil {
		return fmt.Errorf("failed to write config to file: %v", err)
	}

	fmt.Println("Configuration file created successfully.")
	// crae/etc/sheldx/static  exists if not create it and put index.html file with welcome to sheldx message

	// Check if the directory exists, create it if not
	staticDir := "/etc/sheldx/static"
	if _, err := os.Stat(staticDir); os.IsNotExist(err) {
		err := os.MkdirAll(staticDir, 0755)
		if err != nil {
			return fmt.Errorf("failed to create static directory: %v", err)
		}
	}

	// Open the file for writing (create if it doesn't exist, truncate if it does)
	staticFilePath := filepath.Join(staticDir, "index.html")
	staticFile, err := os.OpenFile(staticFilePath, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)

	if err != nil {
		return fmt.Errorf("failed to open static file: %v", err)
	}
	defer staticFile.Close()

	// Write the welcome message to the file

	welcomeMessage := []byte("<h1>Welcome to Sheldx</h1>")
	if _, err := staticFile.Write(welcomeMessage); err != nil {
		return fmt.Errorf("failed to write welcome message to file: %v", err)
	}

	return nil
}
