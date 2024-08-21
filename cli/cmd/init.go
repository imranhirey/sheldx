package cmd

import (
	"sheldx-cli/config"
	"sheldx-cli/utils"

	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
)

var InitCmd = &cobra.Command{
	Use:   "init",
	Short: "Start the sheldx process",
	Run: func(cmd *cobra.Command, args []string) {

		var isRiunning = utils.CheckIfProcessIsRunning("sheldx")
		if isRiunning {
			println("sheldx is already running please stop it first before running init")
			// ask if he wants to stop it

			var stop = utils.AskForConfirmation("Do you want to stop it?", "stop", "no", true)
			if stop {
				var res = stopProcess()
				if res {
					logrus.Info("Process stopped")
				} else {
					logrus.Error("Process could not be stopped")
					return
				}

			}

		} else {
			// check if there is already a config file

			var congigs, error = config.LoadConfig()
			if error != nil {
				logrus.Info("There is no config file in etc -> creating a new one")
			}

			if congigs != nil {
				// tell the user that there is already a config file

				// ask if he wants to overwrite it
				var overwrite = utils.AskForConfirmation("here is already a config file Do you want to overwrite it?", "overwrite", "no", true)
				if overwrite {
					logrus.Info("Overwriting config file")
					// create a new config file
					var jawwab = config.CreateConfigFile()
					if jawwab != nil {
						logrus.Error("Config file could not be created, please try again")
						logrus.Error(jawwab)
						return
					}
				} else {
					// tell the user that the process will not start
					logrus.Info("Process will not start")
					return

				}
			} else {
				// create a new config file

				logrus.Info("Creating a new config file")
				var jawwab = config.CreateConfigFile()
				if jawwab != nil {
					logrus.Error("Config file could not be created, please try again")
					logrus.Error(jawwab)
					return
				}

			}
			return
		}
	},
}

func initSheldx() {

	// check if sheldx is already running

	// check if there iis already config files in etc

}
