package cmd

import (
	"fmt"
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

var StopCmd = &cobra.Command{
	Use:   "stop",
	Short: "Stop the sheldx process",
	Run: func(cmd *cobra.Command, args []string) {
		if success := stopProcess(); !success {
			os.Exit(1)
		}
		fmt.Println("sheldx stopped.")
	},
}

func stopProcess() bool {
	// Corrected command and arguments
	cmd := exec.Command("pkill", "sheldx")

	fmt.Println("Stopping sheldx process...")

	// Run the command and capture the output and error
	err := cmd.Run()
	if err != nil {
		// Print error message and return false
		fmt.Printf("Error stopping process: %v\n", err)
		return false
	}

	// If no error, return true
	return true
}
