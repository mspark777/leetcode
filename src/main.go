package main

import (
	"fmt"
)

func search(nums []int, target int) bool {
	right := len(nums)
	if right < 1 {
		return false
	}

	left := 0
	for left < right {
		mid := (left + right) / 2
		mnum := nums[mid]
		if mnum == target {
			return true
		}

		lnum := nums[left]
		if lnum == mnum {
			left += 1
			continue
		}

		pivotArray := lnum <= mnum
		targetArray := lnum <= target
		if pivotArray != targetArray {
			if pivotArray {
				left = mid + 1
			} else {
				right = mid
			}
		} else {
			if mnum < target {
				left = mid + 1
			} else {
				right = mid
			}
		}
	}

	return false
}

type input struct {
	nums   []int
	target int
}

func main() {
	inputs := []input{
		{nums: []int{2, 5, 6, 0, 0, 1, 2}, target: 0},
		{nums: []int{2, 5, 6, 0, 0, 1, 2}, target: 3},
		{nums: []int{1, 0, 1, 1, 1}, target: 0},
	}

	for _, input := range inputs {
		result := search(input.nums, input.target)
		fmt.Println(result)
	}
}
