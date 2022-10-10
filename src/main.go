package main

import (
	"fmt"
)

func breakPalindrome(palindrome string) string {
	chars := []rune(palindrome)
	slen := len(chars)
	if slen <= 1 {
		return ""
	}

	for i := 0; i < slen/2; i += 1 {
		ch := chars[i]
		if ch != 'a' {
			chars[i] = 'a'
			return string(chars)
		}
	}

	chars[slen-1] = 'b'
	return string(chars)
}

func main() {
	inputs := []string{
		"abccba", "a", "aba",
	}

	for _, input := range inputs {
		result := breakPalindrome(input)
		fmt.Println(result)
	}
}
