package main

import (
	"fmt"
	"sort"
)

func closeStrings(word1 string, word2 string) bool {
	if len(word1) != len(word2) {
		return false
	}

	freq1 := map[byte]int{}
	freq2 := map[byte]int{}
	for i := 0; i < len(word1); i += 1 {
		ch1 := word1[i]
		ch2 := word2[i]

		freq1[ch1] += 1
		freq2[ch2] += 1
	}

	if len(freq1) != len(freq2) {
		return false
	}

	counts1 := []int{}
	counts2 := []int{}
	for k, v1 := range freq1 {
		if v2, ok := freq2[k]; ok {
			counts1 = append(counts1, v1)
			counts2 = append(counts2, v2)
		} else {
			return false
		}
	}

	sort.Sort(sort.Reverse(sort.IntSlice(counts1)))
	sort.Sort(sort.Reverse(sort.IntSlice(counts2)))

	for i, v1 := range counts1 {
		if v1 != counts2[i] {
			return false
		}
	}

	return true
}

func main() {
	inputs := [][]string{
		{"abc", "bca"},
		{"a", "aa"},
		{"cabbba", "abbccc"},
	}

	for _, words := range inputs {
		word1 := words[0]
		word2 := words[1]
		result := closeStrings(word1, word2)
		fmt.Println(result)
	}
}
