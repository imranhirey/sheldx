package cmd

import (
	"github.com/spf13/cobra"
)

var RootCmd = &cobra.Command{
	Use:   "sheldx-cli",
	Short: "A CLI to manage sheldx",
	Long:  `A CLI to start, stop, and restart the sheldx process.`,
}
