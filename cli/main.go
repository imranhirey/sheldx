package main

import (
	"fmt"
	"os"

	"sheldx-cli/cmd" // Import your cmd package

	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
)

func main() {

	logrus.SetLevel(logrus.DebugLevel)
	rootCmd := &cobra.Command{Use: "sheldx-cli"}
	rootCmd.AddCommand(cmd.StartCmd)
	rootCmd.AddCommand(cmd.StopCmd)
	rootCmd.AddCommand(cmd.RestartCmd)
	rootCmd.AddCommand(cmd.InitCmd)
	rootCmd.AddCommand(cmd.StatusCmd) // Add the status command

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

// package main

// import (
// 	"fmt"
// 	c "sheldx-cli/config" // Alias the config package as 'c'
// )

// func main() {
// 	confis, err := c.LoadConfig() // Use the alias to call LoadConfig
// 	if err != nil {
// 		panic(err)
// 	}
// 	fmt.Println(confis)
// }
