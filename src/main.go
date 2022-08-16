package main

import (
	"fmt"
)

func firstUniqChar(s string) int {
	memo := make(map[rune]int)

	for _, ch := range s {
		memo[ch] = memo[ch] + 1
	}

	for i, ch := range s {
		if memo[ch] == 1 {
			return i
		}
	}

	return -1
}

type input struct {
	s string
}

func main() {
	inputs := []*input{
		{
			s: "leetcode",
		},
		{
			s: "loveleetcode",
		},
		{
			s: "aabb",
		},
	}

	for _, input := range inputs {
		result := firstUniqChar(input.s)
		fmt.Println(result)
	}
}
