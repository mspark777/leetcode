package main

import (
	"fmt"
)

func majorityElement(nums []int) int {
	count := 0
	candidate := nums[0]

	for _, num := range nums {
		if count < 1 {
			candidate = num
		}

		if num == candidate {
			count += 1
		} else {
			count -= 1
		}
	}

	return candidate
}

type input struct {
	nums []int
}

func main() {
	inputs := []*input{
		{
			nums: []int{3, 2, 3},
		},
		{
			nums: []int{2, 2, 1, 1, 1, 2, 2},
		},
	}

	for _, input := range inputs {
		result := majorityElement(input.nums)
		fmt.Println(result)
	}
}
