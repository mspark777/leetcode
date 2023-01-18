package main

import (
	"fmt"
)

func max(a, b int) int {
	if a < b {
		return b
	}

	return a
}

func min(a, b int) int {
	if a < b {
		return a
	}

	return b
}

func maxSubarraySumCircular(nums []int) int {
	curmax := 0
	curmin := 0
	sum := 0
	maxsum := nums[0]
	minsum := nums[0]

	for _, num := range nums {
		curmax = max(curmax, 0) + num
		curmin = min(curmin, 0) + num
		maxsum = max(curmax, maxsum)
		minsum = min(curmin, minsum)
		sum += num
	}

	if sum == minsum {
		return maxsum
	}

	return max(maxsum, sum-minsum)
}

func main() {
	inputs := [][]int{
		{1, -2, 3, -2},
		{5, -3, 5},
		{-3, -2, -3},
	}

	for _, nums := range inputs {
		result := maxSubarraySumCircular(nums)
		fmt.Println(result)
	}
}
