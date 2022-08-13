package main

import (
	"fmt"
)

func findSubstring(s string, words []string) []int {
	wordsLen := len(words)
	wordLen := len(words[0])

	result := []int{}
	wordCount := make(map[string]int)
	for _, word := range words {
		wordCount[word] += 1
	}

	lastWindowIndex := len(s) - (wordsLen * wordLen)
	for i := 0; i <= lastWindowIndex; i += 1 {
		twordCount := make(map[string]int)
		for k, v := range wordCount {
			twordCount[k] = v
		}

		for j := i; (j < len(s)) && (len(twordCount) > 0); j += wordLen {
			str := s[j : j+wordLen]
			cnt := twordCount[str]
			if cnt == 0 {
				break
			} else {
				if cnt == 1 {
					delete(twordCount, str)
				} else {
					twordCount[str] = cnt - 1
				}
			}
		}

		if len(twordCount) == 0 {
			result = append(result, i)
		}
	}

	return result
}

type input struct {
	s     string
	words []string
}

func main() {
	inputs := []*input{
		{
			s:     "barfoothefoobarman",
			words: []string{"foo", "bar"},
		},
		{
			s:     "wordgoodgoodgoodbestword",
			words: []string{"word", "good", "best", "word"},
		},
		{
			s:     "barfoofoobarthefoobarman",
			words: []string{"bar", "foo", "the"},
		},
	}

	for _, input := range inputs {
		s := input.s
		words := input.words
		result := findSubstring(s, words)
		fmt.Println(result)
	}
}
