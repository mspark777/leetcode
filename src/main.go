package main

import (
	"fmt"
	"strings"
)

func detectCapitalUse(word string) bool {
	if word == strings.ToUpper(word) {
		return true
	} else if word == strings.ToLower(word) {
		return true
	}

	return word == strings.ToUpper(string(word[0]))+strings.ToLower(string(word[1:]))
}

func main() {
	inputs := []string{
		"USA",
		"Google",
		"leetcode",
		"FlaG",
	}

	for _, word := range inputs {
		result := detectCapitalUse(word)
		fmt.Println(result)
	}
}
