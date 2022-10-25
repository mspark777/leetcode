package main

import (
	"fmt"
	"strings"
)

func arrayStringsAreEqual(word1 []string, word2 []string) bool {
	return strings.Join(word1, "") == strings.Join(word2, "")
}

type input struct {
	word1 []string
	word2 []string
}

func main() {
	inputs := []input{
		{
			word1: []string{"ab", "c"},
			word2: []string{"a", "bc"},
		},
		{
			word1: []string{"a", "cb"},
			word2: []string{"ab", "c"},
		},
		{
			word1: []string{"abc", "d", "defg"},
			word2: []string{"abcddefg"},
		},
		{
			word1: []string{"abc", "d", "defg"},
			word2: []string{"abcddef"},
		},
	}

	for _, input := range inputs {
		word1 := input.word1
		word2 := input.word2
		result := arrayStringsAreEqual(word1, word2)
		fmt.Println(result)
	}
}
