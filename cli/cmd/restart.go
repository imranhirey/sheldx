package cmd

import (
	"github.com/spf13/cobra"
)

var RestartCmd = &cobra.Command{
	Use:   "restart",
	Short: "Restart the sheldx process",
	Run: func(cmd *cobra.Command, args []string) {
		restartProcess()
	},
}

func restartProcess() error {

	println("sheldx stopped.")
	// start again

	startProcess()

	return startProcess()

}
