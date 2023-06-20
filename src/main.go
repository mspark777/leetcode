package main

import (
	"fmt"
)

func getAverages(nums []int, k int) []int {
	if k < 1 {
		return nums
	}

	result := make([]int, len(nums))
	for i := range nums {
		result[i] = -1
	}

	windowLen := (k * 2) + 1
	if windowLen > len(nums) {
		return result
	}

	windowSum := 0
	for i := 0; i < windowLen; i += 1 {
		windowSum += nums[i]
	}
	result[k] = windowSum / windowLen

	for i := windowLen; i < len(nums); i += 1 {
		windowSum = windowSum - nums[i-windowLen] + nums[i]
		result[i-k] = windowSum / windowLen
	}

	return result
}

type input struct {
	nums []int
	k    int
}

func main() {
	inputs := []input{
		{nums: []int{7, 4, 3, 9, 1, 8, 5, 2, 6}, k: 3},
		{nums: []int{100000}, k: 0},
		{nums: []int{8}, k: 100000},
	}

	for _, input := range inputs {
		result := getAverages(input.nums, input.k)
		fmt.Println(result)
	}
}
