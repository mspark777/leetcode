package main

import (
	"fmt"
	"strings"
)

func reverseWords(s string) string {
	words := strings.Split(s, " ")
	result := make([]string, len(words))

	for i, word := range words {
		bytes := []rune(word)
		j := 0
		k := len(bytes) - 1
		for j < k {
			bytes[j], bytes[k] = bytes[k], bytes[j]
			j += 1
			k -= 1
		}

		result[i] = string(bytes)
	}

	return strings.Join(result, " ")
}

type input struct {
	s string
}

func main() {
	inputs := []input{
		{
			s: "Let's take LeetCode contest",
		},
		{
			s: "God Ding",
		},
	}

	for _, input := range inputs {
		s := input.s
		result := reverseWords(s)
		fmt.Println(result)
	}
}
