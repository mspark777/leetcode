package main

import (
	"fmt"
)

func countWords(words1 []string, words2 []string) int {
	counts := make(map[string]int)
	for _, word := range words1 {
		counts[word] += 1
	}

	for _, word := range words2 {
		if count, ok := counts[word]; ok {
			if count < 2 {
				counts[word] -= 1
			}
		}
	}

	result := 0
	for _, count := range counts {
		if count == 0 {
			result += 1
		}
	}
	return result
}

type input struct {
	words1 []string
	words2 []string
}

func main() {
	inputs := []input{
		{
			words1: []string{"leetcode", "is", "amazing", "as", "is"},
			words2: []string{"amazing", "leetcode", "is"},
		},
		{
			words1: []string{"b", "bb", "bbb"},
			words2: []string{"a", "aa", "aaa"},
		},
		{
			words1: []string{"a", "ab"},
			words2: []string{"a", "a", "a", "ab"},
		},
	}

	for _, input := range inputs {
		result := countWords(input.words1, input.words2)
		fmt.Println(result)
	}
}
