package main

import (
	"fmt"
)

func singleNonDuplicate(nums []int) int {
	left := 0
	right := len(nums) - 1

	for left < right {
		middle := (left + right) / 2
		if (middle & 1) == 1 {
			if nums[middle] != nums[middle+1] {
				left = middle + 1
			} else {
				right = middle
			}
		} else {
			if nums[middle] == nums[middle+1] {
				left = middle + 1
			} else {
				right = middle
			}
		}
	}

	return nums[left]
}

func main() {
	inputs := [][]int{
		{1, 1, 2, 3, 3, 4, 4, 8, 8},
		{3, 3, 7, 7, 10, 11, 11},
	}

	for _, input := range inputs {
		result := singleNonDuplicate(input)
		fmt.Println(result)
	}
}
