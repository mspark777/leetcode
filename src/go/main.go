package main

import (
	"fmt"
	"sort"
)

func minDifference(nums []int) int {
	n := len(nums)
	if n <= 4 {
		return 0
	}

	sort.Ints(nums)

	result := 0x7FFFFFFFFFFFFFFF

	left := 0
	right := n - 4
	for left < 4 {
		diff := nums[right] - nums[left]
		if diff < result {
			result = diff
		}

		left += 1
		right += 1
	}

	return result
}

type input struct {
	nums []int
}

func main() {
	inputs := []input{
		{
			[]int{5, 3, 2, 4},
		},
		{
			[]int{1, 5, 0, 10, 14},
		},
		{
			[]int{3, 100, 20},
		},
	}

	for _, input := range inputs {
		result := minDifference(input.nums)
		fmt.Println(result)
	}
}
