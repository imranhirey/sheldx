package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var startCmd = &cobra.Command{
	Use:   "start",
	Short: "Start the SheldX proxy server",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Starting SheldX...")
		// Add your start logic here
	},
}

func init() {
	rootCmd.AddCommand(startCmd)
}
