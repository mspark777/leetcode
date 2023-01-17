package main

import (
	"fmt"
)

func isSubsequence(s string, t string) bool {
	if s == "" {
		return true
	}

	si := 0

	for _, c := range t {
		if rune(s[si]) == c {
			si += 1
		}

		if len(s) == si {
			return true
		}
	}

	return false
}

func main() {
	inputs := [][]string{
		{"abc", "ahbgdc"}, {"axc", "ahbgdc"},
		{"", ""}, {"", "ahbgdc"},
	}

	for _, input := range inputs {
		result := isSubsequence(input[0], input[1])
		fmt.Println(result)
	}
}
