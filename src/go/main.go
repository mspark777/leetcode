package main

import (
	"fmt"
)

func runningSum(nums []int) []int {
	result := make([]int, len(nums))
	result[0] = nums[0]

	for i, n := range nums[1:] {
		result[i+1] = result[i] + n
	}

	return result
}

type input struct {
	nums []int
}

func main() {
	inputs := []input{
		{
			nums: []int{1, 2, 3, 4},
		},
		{
			nums: []int{1, 1, 1, 1, 1},
		},
		{
			nums: []int{3, 1, 2, 10, 1},
		},
	}

	for _, input := range inputs {
		result := runningSum(input.nums)
		fmt.Println(result)
	}
}
