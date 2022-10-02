package main

import (
	"fmt"
)

func numRollsToTarget(n int, k int, target int) int {
	const MOD = 1000000007
	dp := make([]int, target+1)
	dp[0] = 1

	for i := 1; i <= n; i += 1 {
		for j := target; j >= 0; j -= 1 {
			dp[j] = 0

			for p := 1; p <= k; p += 1 {
				if j >= p {
					dp[j] = (dp[j] + dp[j-p]) % MOD
				} else {
					break
				}
			}
		}
	}

	return dp[target]
}

type input struct {
	n, k, target int
}

func main() {
	inputs := []input{
		{
			n:      1,
			k:      6,
			target: 3,
		},
		{
			n:      2,
			k:      6,
			target: 7,
		},
		{
			n:      30,
			k:      30,
			target: 500,
		},
	}

	for _, input := range inputs {
		n := input.n
		k := input.k
		target := input.target
		result := numRollsToTarget(n, k, target)
		fmt.Println(result)
	}
}
