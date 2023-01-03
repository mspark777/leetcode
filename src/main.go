package main

import (
	"fmt"
)

func search(nums []int, target int) int {
	left := 0
	right := len(nums) - 1
	for left <= right {
		mid := (left + right) / 2
		if nums[mid] == target {
			return mid
		}

		if nums[mid] >= nums[left] {
			if (target >= nums[left]) && (target < nums[mid]) {
				right = mid - 1
			} else {
				left = mid + 1
			}
		} else if (target > nums[mid]) && (target <= nums[right]) {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}

	return -1
}

type input struct {
	nums   []int
	target int
}

func main() {
	inputs := []input{
		{nums: []int{4, 5, 6, 7, 0, 1, 2}, target: 0},
		{nums: []int{4, 5, 6, 7, 0, 1, 2}, target: 3},
		{nums: []int{1}, target: 0},
	}

	for _, input := range inputs {
		result := search(input.nums, input.target)
		fmt.Println(result)
	}
}
