package main

import (
	"fmt"
)

func strStr(haystack string, needle string) int {
	m := len(needle)
	n := len(haystack)

	for start := 0; start <= n-m; start += 1 {
		for i := 0; i < m; i += 1 {
			if needle[i] != haystack[start+i] {
				break
			}

			if i == (m - 1) {
				return start
			}
		}
	}

	return -1
}

func main() {
	inputs := [][]string{
		{"sadbutsad", "sad"},
		{"leetcode", "leeto"},
	}

	for _, input := range inputs {
		result := strStr(input[0], input[1])
		fmt.Println(result)
	}
}
