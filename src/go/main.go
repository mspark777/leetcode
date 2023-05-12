package main

import (
	"fmt"
)

func mostPoints(questions [][]int) int64 {
	qlen := len(questions)
	last := qlen - 1
	dp := make([]int, qlen)
	dp[last] = questions[last][0]

	for i := last - 1; i >= 0; i -= 1 {
		question := questions[i]
		dp[i] = question[0]
		power := question[1]

		if (i + power) < last {
			dp[i] += dp[i+power+1]
		}

		j := i + 1
		if dp[j] > dp[i] {
			dp[i] = dp[j]
		}
	}

	return int64(dp[0])
}

func main() {
	inputs := [][][]int{
		{{3, 2}, {4, 3}, {4, 4}, {2, 5}},
		{{1, 1}, {2, 2}, {3, 3}, {4, 4}, {5, 5}},
	}

	for _, input := range inputs {
		result := mostPoints(input)
		fmt.Println(result)
	}
}
