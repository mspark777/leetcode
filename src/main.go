package main

import (
	"fmt"
	"strings"
)

func uniqueMorseRepresentations(words []string) int {
	morses := []string{
		".-", "-...", "-.-.", "-..", ".", "..-.", "--.",
		"....", "..", ".---", "-.-", ".-..", "--", "-.",
		"---", ".--.", "--.-", ".-.", "...", "-", "..-",
		"...-", ".--", "-..-", "-.--", "--..",
	}

	seen := make(map[string]bool)
	for _, word := range words {
		codes := []string{}
		for _, ch := range word {
			i := ch - rune('a')
			codes = append(codes, morses[i])
		}

		morse := strings.Join(codes, "")
		seen[morse] = true
	}

	return len(seen)
}

type input struct {
	words []string
}

func main() {
	inputs := []*input{
		{
			words: []string{"gin", "zen", "gig", "msg"},
		},
		{
			words: []string{"a"},
		},
	}

	for _, input := range inputs {
		words := input.words
		result := uniqueMorseRepresentations(words)
		fmt.Println(result)
	}
}
