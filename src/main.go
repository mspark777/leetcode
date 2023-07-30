package main

import (
	"fmt"
)

func strangePrinter(s string) int {
	bs := []byte(s)
	n := len(bs)
	dp := make([][]int, n)
	for i := 0; i < n; i += 1 {
		dp[i] = make([]int, n)
	}

	for l := 1; l <= n; l += 1 {
		for left := 0; left <= n-l; left += 1 {
			right := left + l - 1
			dp[left][right] = n

			j := -1
			for i := left; i < right; i += 1 {
				if (bs[i] != bs[right]) && (j == -1) {
					j = i
				}

				if j != -1 {
					lmin := dp[left][right]
					rmin := 1 + dp[j][i] + dp[i+1][right]
					if lmin < rmin {
						dp[left][right] = lmin
					} else {
						dp[left][right] = rmin
					}
				}
			}

			if j == -1 {
				dp[left][right] = 0
			}
		}
	}

	return dp[0][n-1] + 1
}

func main() {
	inputs := []string{
		"aaabbb",
		"aba",
	}

	for _, input := range inputs {
		result := strangePrinter(input)
		fmt.Println(result)
	}
}
