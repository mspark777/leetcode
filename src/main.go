package main

import (
	"fmt"
)

func reverseVowels(s string) string {
	vowels := map[rune]bool{
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

	words := []rune(s)
	left := 0
	right := len(words) - 1

	for left < right {
		l := words[left]
		if _, ok := vowels[l]; !ok {
			left += 1
			continue
		}

		r := words[right]
		if _, ok := vowels[r]; !ok {
			right -= 1
			continue
		}

		words[left] = r
		words[right] = l
		left += 1
		right -= 1
	}

	return string(words)
}

func main() {
	inputs := []string{
		"hello",
		"leetcode",
	}

	for _, s := range inputs {
		result := reverseVowels(s)
		fmt.Println(result)
	}
}
