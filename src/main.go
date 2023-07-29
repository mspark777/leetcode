package main

import (
	"fmt"
	"math"
)

func max(i, j int) int {
	if i < j {
		return j
	}

	return i
}

func calculateDP(i, j int, dp map[int]map[int]float64) float64 {
	dp0 := dp[max(0, i-4)][j]
	dp1 := dp[max(0, i-3)][j-1]
	dp2 := dp[max(0, i-2)][max(0, j-2)]
	dp3 := dp[i-1][max(0, j-3)]
	sum := dp0 + dp1 + dp2 + dp3
	return sum / 4
}

func soupServings(n int) float64 {
	dp := map[int]map[int]float64{}
	dp[0] = map[int]float64{0: 0.5}

	m := int(math.Ceil(float64(n) / 25.0))
	for k := 1; k <= m; k += 1 {
		dp[k] = map[int]float64{0: 0}
		dp[0][k] = 1

		for j := 1; j <= k; j += 1 {
			dp[j][k] = calculateDP(j, k, dp)
			dp[k][j] = calculateDP(k, j, dp)

			if dp[k][k] > (1 - 1e-5) {
				return 1
			}
		}
	}

	return dp[m][m]
}

func main() {
	inputs := []int{
		50, 100,
	}

	for _, input := range inputs {
		result := soupServings(input)
		fmt.Println(result)
	}
}
