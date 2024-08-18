package cmd

import (
	"os"

	"github.com/spf13/cobra"
)

var stopCmd = &cobra.Command{
	Use:   "stop",
	Short: "Stop the SheldX proxy server",
	Run: func(cmd *cobra.Command, args []string) {
		println("trying  to stop sheldx server")
		// Add your stop logic here
		println("sheldx server stopped successfully")
	},
}
var test = &cobra.Command{
	Use:   "test",
	Short: "test",
	Run: func(cmd *cobra.Command, args []string) {
		testw()
	},
}

func init() {
	rootCmd.AddCommand(stopCmd)
	rootCmd.AddCommand(test)
}

func testw() {
	// check if sheldx in etc folder

	var path = "/etc/sheldx"
	if _, err := os.Stat(path); os.IsNotExist(err) {
		println("sheldx not found")
	} else {
		println("sheldx found")
	}
}
