package main

import (
	"fmt"
)

func new21Game(n int, k int, maxPts int) float64 {
	if k == 0 {
		return 1.0
	} else if n >= (k + maxPts) {
		return 1.0
	}

	dp := make([]float64, n+1)
	dp[0] = 1.0
	sum := 1.0
	result := 0.0
	for i := 1; i <= n; i += 1 {
		dp[i] = sum / float64(maxPts)
		if i < k {
			sum += dp[i]
		} else {
			result += dp[i]
		}

		if (i - maxPts) >= 0 {
			sum -= dp[i-maxPts]
		}
	}

	return result
}

func main() {
	inputs := [][]int{
		{10, 1, 10},
		{6, 1, 10},
		{21, 17, 10},
	}

	for _, input := range inputs {
		result := new21Game(input[0], input[1], input[2])
		fmt.Println(result)
	}
}
