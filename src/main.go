package main

import (
	"fmt"
	"strings"
)

func wordPattern(pattern string, s string) bool {
	words := strings.Split(s, " ")
	patterns := strings.Split(pattern, "")

	if len(words) != len(patterns) {
		return false
	}

	ptow := make(map[string]string)
	wtop := make(map[string]string)

	for i, word := range words {
		ptn := patterns[i]
		if w, ok := ptow[ptn]; ok {
			if w != word {
				return false
			}
		} else {
			ptow[ptn] = word
		}

		if p, ok := wtop[word]; ok {
			if p != ptn {
				return false
			}
		} else {
			wtop[word] = ptn
		}
	}

	return true
}

func main() {
	inputs := [][]string{
		{"abba", "dog cat cat dog"},
		{"abba", "dog cat cat fish"},
		{"aaaa", "dog cat cat dog"},
		{"abba", "dog dog dog dog"},
	}

	for _, input := range inputs {
		result := wordPattern(input[0], input[1])
		fmt.Println(result)
	}
}
