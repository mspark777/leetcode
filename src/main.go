package main

import (
	"fmt"
)

func searchInsert(nums []int, target int) int {
	begin := 0
	if target < nums[begin] {
		return 0
	}

	end := len(nums)
	if target > nums[end-1] {
		return end
	}

	for begin < end {
		middle := (begin + end) / 2
		num := nums[middle]
		if num < target {
			begin = middle + 1
		} else if num > target {
			end = middle
		} else {
			return middle
		}
	}

	return begin
}

type input struct {
	nums   []int
	target int
}

func main() {
	inputs := []input{
		{nums: []int{1, 3, 5, 6}, target: 5},
		{nums: []int{1, 3, 5, 6}, target: 2},
		{nums: []int{1, 3, 5, 6}, target: 7},
	}

	for _, input := range inputs {
		result := searchInsert(input.nums, input.target)
		fmt.Println(result)
	}
}
