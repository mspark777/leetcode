package main

import (
	"fmt"
)

type input struct {
	words   []string
	pattern string
}

func main() {
	inputs := []*input{
		{
			words:   []string{"abc", "deq", "mee", "aqq", "dkd", "ccc"},
			pattern: "abb",
		},
		{
			words:   []string{"a", "b", "c"},
			pattern: "a",
		},
	}

	for _, input := range inputs {
		result := findAndReplacePattern(input.words, input.pattern)
		fmt.Printf("%v\n", result)
	}
}
