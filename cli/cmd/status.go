package cmd

import (
	"fmt"
	"sheldx-cli/utils"

	"github.com/spf13/cobra"
)

var StatusCmd = &cobra.Command{
	Use:   "status",
	Short: "Check the status of the sheldx process",
	Run: func(cmd *cobra.Command, args []string) {
		if err := checkStatus(); err != nil {
			fmt.Println("Error checking status:", err)
			return
		}
	},
}

func checkStatus() error {
	var res = utils.CheckIfProcessIsRunning("sheldx")
	if res == true {
		fmt.Println("sheldx is running.")
	} else {
		fmt.Println("sheldx is not running.")

	}
	return nil
}
