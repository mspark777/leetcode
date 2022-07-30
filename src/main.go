package main

import (
	"fmt"
)

type input struct {
	words1 []string
	words2 []string
}

func main() {
	inputs := []*input{
		{
			words1: []string{"amazon", "apple", "facebook", "google", "leetcode"},
			words2: []string{"e", "o"},
		},
		{
			words1: []string{"amazon", "apple", "facebook", "google", "leetcode"},
			words2: []string{"l", "e"},
		},
	}

	for _, input := range inputs {
		result := wordSubsets(input.words1, input.words2)
		fmt.Printf("%v\n", result)
	}
}
