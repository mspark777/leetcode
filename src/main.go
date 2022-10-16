package main

import (
	"fmt"
)

func max(i, j int) int {
	if i < j {
		return j
	}

	return i
}

func min(i, j int) int {
	if i < j {
		return i
	}

	return j
}

func minDifficulty(jobDifficulty []int, d int) int {
	days := len(jobDifficulty)
	if days < d {
		return -1
	}

	dp := make([]int, days+1)
	for i := days - 1; i >= 0; i -= 1 {
		dp[i] = max(dp[i+1], jobDifficulty[i])
	}

	for i := 2; i <= d; i += 1 {
		remain := days - i
		for j := 0; j <= remain; j += 1 {
			maxd := 0
			dp[j] = 2147483647
			for k := j; k <= remain; k += 1 {
				maxd = max(maxd, jobDifficulty[k])
				dp[j] = min(dp[j], maxd+dp[k+1])
			}
		}
	}

	return dp[0]
}

type input struct {
	jobDifficulty []int
	d             int
}

func main() {
	inputs := []input{
		{jobDifficulty: []int{6, 5, 4, 3, 2, 1}, d: 2},
		{jobDifficulty: []int{9, 9, 9}, d: 4},
		{jobDifficulty: []int{1, 1, 1}, d: 3},
	}

	for _, input := range inputs {
		jobDifficulty := input.jobDifficulty
		d := input.d
		result := minDifficulty(jobDifficulty, d)
		fmt.Println(result)
	}
}
