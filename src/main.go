package main

import (
	"fmt"
)

func minSubArrayLen(target int, nums []int) int {
	left := 0
	sum := 0
	result := 0x8FFFFFFF

	for right, num := range nums {
		sum += num
		for sum >= target {
			l := right - left + 1
			if l < result {
				result = l
			}

			sum -= nums[left]
			left += 1
		}
	}

	if result >= 0x8FFFFFFF {
		result = 0
	}

	return result
}

type input struct {
	target int
	nums   []int
}

func main() {
	inputs := []input{
		{target: 7, nums: []int{2, 3, 1, 2, 4, 3}},
		{target: 4, nums: []int{1, 4, 4}},
		{target: 11, nums: []int{1, 1, 1, 1, 1, 1, 1, 1}},
	}

	for _, input := range inputs {
		result := minSubArrayLen(input.target, input.nums)
		fmt.Println(result)
	}
}
