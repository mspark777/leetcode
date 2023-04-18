package main

import (
	"fmt"
)

func mergeAlternately(word1 string, word2 string) string {
	len1 := len(word1)
	len2 := len(word2)
	rlen := len1 + len2
	maxlen := len1
	if len2 > len1 {
		maxlen = len2
	}

	pos := 0
	result := make([]byte, rlen)
	for i := 0; i < maxlen; i += 1 {
		if i < len1 {
			result[pos] = word1[i]
			pos += 1
		}

		if i < len2 {
			result[pos] = word2[i]
			pos += 1
		}
	}

	return string(result)
}

func main() {
	inputs := [][]string{
		{"abc", "pqr"},
		{"ab", "pqrs"},
		{"abcd", "pq"},
	}

	for _, input := range inputs {
		result := mergeAlternately(input[0], input[1])
		fmt.Println(result)
	}
}
