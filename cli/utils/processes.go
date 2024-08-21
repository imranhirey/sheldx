package utils

import (
	"os/exec"
	"strings"
)

// CheckIfProcessIsRunning checks if a process with the exact name is running.
func CheckIfProcessIsRunning(processName string) bool {
	// Command to list all processes with their names
	cmd := exec.Command("sh", "-c", "ps -eo pid,comm")

	// Run the command and get the output
	output, err := cmd.Output()
	if err != nil {
		return false
	}

	// Convert output to string and split into lines
	outputStr := string(output)
	lines := strings.Split(outputStr, "\n")

	// Iterate through each line to find the exact process name
	for _, line := range lines {
		// Split the line into fields
		fields := strings.Fields(line)
		if len(fields) > 1 {
			// Check if the second field (process name) is an exact match
			if fields[1] == processName {
				return true
			}
		}
	}

	return false
}
