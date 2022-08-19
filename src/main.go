package main

import (
	"fmt"
)

func moveZeroes(nums []int) {
	lastZero := 0

	for i := 0; i < len(nums); i += 1 {
		if nums[i] != 0 {
			nums[lastZero] = nums[i]
			lastZero += 1
		}
	}

	for i := lastZero; i < len(nums); i += 1 {
		nums[i] = 0
	}
}

type input struct {
	nums []int
}

func main() {
	inputs := []*input{
		{
			nums: []int{0, 1, 0, 3, 12},
		},
		{
			nums: []int{0},
		},
	}

	for _, input := range inputs {
		nums := input.nums
		moveZeroes(nums)
		fmt.Println(nums)
	}
}
