package main

import (
	"fmt"
)

func checkIfPangram(sentence string) bool {
	const ACODE = rune('a')
	bits := 0

	for _, code := range []rune(sentence) {
		offset := code - ACODE
		bit := 1 << offset

		bits |= bit
	}

	return bits == 0x03ffffff
}

func main() {
	inputs := []string{
		"thequickbrownfoxjumpsoverthelazydog",
		"leetcode",
	}

	for _, input := range inputs {
		result := checkIfPangram(input)
		fmt.Println(result)
	}
}
