package main

import (
	"fmt"
)

func longestPalindrome(words []string) int {
	counts := map[string]int{}
	for _, word := range words {
		counts[word] += 1
	}

	result := 0
	central := false

	for word, count := range counts {
		first := word[0]
		second := word[1]
		if first == second {
			if (count % 2) == 0 {
				result += count
			} else {
				result += count - 1
				central = true
			}
		} else if first < second {
			rword := string([]byte{second, first})
			if rcount, ok := counts[rword]; ok {
				if count < rcount {
					result += 2 * count
				} else {
					result += 2 * rcount
				}
			}
		}
	}

	if central {
		result += 1
	}

	return result * 2
}

func main() {
	inputs := [][]string{
		{"lc", "cl", "gg"},
		{"ab", "ty", "yt", "lc", "cl", "ab"},
		{"cc", "ll", "xx"},
	}

	for _, words := range inputs {
		result := longestPalindrome(words)
		fmt.Println(result)
	}
}
