package cmd

import (
	"bytes"
	"fmt"
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

var StartCmd = &cobra.Command{
	Use:   "start",
	Short: "Start the sheldx process",
	Run: func(cmd *cobra.Command, args []string) {
		if err := startProcess(); err != nil {
			fmt.Println("Error starting process:", err)
			os.Exit(1)
		}
		fmt.Println("sheldx started successfully.")
	},
}

func startProcess() error {
	// Initialize the command to run the sheldx process
	cmd := exec.Command("/usr/local/bin/sheldx")

	// Buffer to capture stderr
	var stderr bytes.Buffer
	cmd.Stderr = &stderr

	// Start the command
	if err := cmd.Start(); err != nil {
		return fmt.Errorf("failed to start sheldx: %v", err)
	}

	// Wait for the command to finish
	if err := cmd.Wait(); err != nil {
		// Return the error along with the captured stderr output
		return fmt.Errorf("sheldx process exited with error: %v, details: %s", err, stderr.String())
	}

	// Return nil if everything is successful
	return nil
}

// if there is error
