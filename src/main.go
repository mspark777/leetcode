package main

import (
	"fmt"
)

func wordBreak(s string, wordDict []string) bool {
	words := make(map[string]bool)
	for _, w := range wordDict {
		words[w] = true
	}

	checks := make([]bool, len(s)+1)
	checks[0] = true
	for right := 1; right <= len(s); right += 1 {
		for left := 0; left < right; left += 1 {
			if !checks[left] {
				continue
			}

			sub := s[left:right]
			if _, ok := words[sub]; ok {
				checks[right] = true
				break
			}
		}
	}

	return checks[len(s)]
}

type input struct {
	s        string
	wordDict []string
}

func main() {
	inputs := []input{
		{s: "leetcode", wordDict: []string{"leet", "code"}},
		{s: "applepenapple", wordDict: []string{"apple", "pen"}},
		{s: "catsandog", wordDict: []string{"cats", "dog", "sand", "and", "cat"}},
	}

	for _, input := range inputs {
		result := wordBreak(input.s, input.wordDict)
		fmt.Println(result)
	}
}
