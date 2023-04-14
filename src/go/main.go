package main

import (
	"fmt"
)

func longestPalindromeSubseq(s string) int {
	n := len(s)
	dp := make([]int, n)
	dpPrev := make([]int, n)

	for i := n - 1; i >= 0; i -= 1 {
		dp[i] = 1
		for j := i + 1; j < n; j += 1 {
			if s[i] == s[j] {
				dp[j] = dpPrev[j-1] + 2
			} else {
				prev := dpPrev[j]
				cur := dp[j-1]
				if prev > cur {
					dp[j] = prev
				} else {
					dp[j] = cur
				}
			}
		}
		copy(dpPrev, dp)
	}

	return dp[n-1]
}

func main() {
	inputs := []string{
		"bbbab",
		"cbbd",
	}

	for _, s := range inputs {
		result := longestPalindromeSubseq(s)
		fmt.Println(result)
	}
}
