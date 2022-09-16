package main

import (
	"fmt"
)

func maximumScore(nums []int, multipliers []int) int {
	n := len(nums)
	m := len(multipliers)
	dp := make([]int, m+1)

	for op := m - 1; op >= 0; op -= 1 {
		row := make([]int, m+1)
		copy(row, dp)

		for left := op; left >= 0; left -= 1 {
			n0 := multipliers[op]*nums[left] + row[left+1]
			n1 := multipliers[op]*nums[n-1-(op-left)] + row[left]
			if n0 > n1 {
				dp[left] = n0
			} else {
				dp[left] = n1
			}
		}
	}

	return dp[0]
}

type input struct {
	nums        []int
	multipliers []int
}

func main() {
	inputs := []input{
		{
			nums:        []int{1, 2, 3},
			multipliers: []int{3, 2, 1},
		},
		{
			nums:        []int{-5, -3, -3, -2, 7, 1},
			multipliers: []int{-10, -5, 3, 4, 6},
		},
	}

	for _, input := range inputs {
		nums := input.nums
		multipliers := input.multipliers
		result := maximumScore(nums, multipliers)
		fmt.Println(result)
	}
}
