package main

import (
	"fmt"
)

func minDeletionSize(strs []string) int {
	result := 0

	for c := 0; c < len(strs[0]); c += 1 {
		for r := 1; r < len(strs); r += 1 {
			c0 := strs[r-1][c]
			c1 := strs[r][c]
			if c0 > c1 {
				result += 1
				break
			}
		}
	}

	return result
}

func main() {
	inputs := [][]string{
		{"cba", "daf", "ghi"},
		{"a", "b"},
		{"zyx", "wvu", "tsr"},
	}

	for _, strs := range inputs {
		result := minDeletionSize(strs)
		fmt.Println(result)
	}
}
