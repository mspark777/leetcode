package main

import (
	"fmt"
)

type input struct {
	s     string
	words []string
}

func main() {
	inputs := []input{
		{
			s:     "abcde",
			words: []string{"a", "bb", "acd", "ace"},
		},
		{
			s:     "dsahjpjauf",
			words: []string{"ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"},
		},
	}

	for _, input := range inputs {
		result := numMatchingSubseq(input.s, input.words)
		fmt.Printf("%v\n", result)
	}
}
