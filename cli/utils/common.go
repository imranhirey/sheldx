package utils

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

// AskForConfirmation prompts the user with a message and expects a confirmation or denial response.
func AskForConfirmation(message string, confirmingWord string, denyingWord string, caseSensitive bool) bool {
	// Create a new reader to read input from standard input
	reader := bufio.NewReader(os.Stdin)

	// Prompt the user
	fmt.Printf("%s [%s/%s] or type no/NO to cancel: ", message, confirmingWord, strings.ToUpper(confirmingWord))

	// Read the input from the user
	input, err := reader.ReadString('\n')
	if err != nil {
		fmt.Println("Error reading input:", err)
		return false
	}

	// Trim any leading or trailing whitespace
	input = strings.TrimSpace(input)

	// Check if the input matches the confirmingWord
	if caseSensitive {
		if input == confirmingWord {
			return true
		}
	} else {
		if strings.EqualFold(input, confirmingWord) {
			return true
		}
	}

	// Check if the input matches the denyingWord
	if caseSensitive {
		if input == denyingWord {
			return false
		}
	} else {
		if strings.EqualFold(input, denyingWord) {
			return false
		}
	}

	// Return false by default if the input is neither confirmingWord nor denyingWord
	fmt.Printf("Invalid input. Please enter either '%s' or '%s'.\n", confirmingWord, denyingWord)
	return false
}
