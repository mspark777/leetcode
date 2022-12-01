package main

import (
	"fmt"
)

func halvesAreAlike(s string) bool {
	vowels := map[byte]bool{
		'a': true,
		'e': true,
		'i': true,
		'o': true,
		'u': true,
		'A': true,
		'E': true,
		'I': true,
		'O': true,
		'U': true,
	}

	first := 0
	second := 0

	i := 0
	for j := len(s) / 2; j < len(s); j += 1 {
		if _, ok := vowels[s[i]]; ok {
			first += 1
		}

		if _, ok := vowels[s[j]]; ok {
			second += 1
		}

		i += 1
	}

	return first == second
}

func main() {
	inputs := []string{
		"book",
		"textbook",
	}

	for _, s := range inputs {
		result := halvesAreAlike(s)
		fmt.Println(result)
	}
}
