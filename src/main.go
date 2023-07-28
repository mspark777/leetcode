package main

import (
	"fmt"
)

func PredictTheWinner(nums []int) bool {
	n := len(nums)
	dp := make([]int, n)
	copy(dp, nums)

	for diff := 1; diff < n; diff += 1 {
		for left := 0; left < n-diff; left += 1 {
			right := left + diff
			lnum := nums[left]
			rnum := nums[right]
			ldp := dp[left]
			rdp := dp[left+1]
			l := lnum - rdp
			r := rnum - ldp
			if l < r {
				dp[left] = r
			} else {
				dp[left] = l
			}
		}
	}

	return dp[0] >= 0
}

func main() {
	inputs := [][]int{
		{1, 5, 2},
		{1, 5, 233, 7},
	}

	for _, input := range inputs {
		result := PredictTheWinner(input)
		fmt.Println(result)
	}
}
