package main

import (
	"fmt"
)

func firstMissingPositive(nums []int) int {
	for i := 0; i < len(nums); i += 1 {
		for (nums[i] > 0) && (nums[i] <= len(nums)) && (nums[nums[i]-1] != nums[i]) {
			temp := nums[i]
			nums[i] = nums[temp-1]
			nums[temp-1] = temp
		}
	}

	for i, num := range nums {
		if num != (i + 1) {
			return i + 1
		}
	}

	return len(nums) + 1
}

func main() {
	inputs := [][]int{
		{1, 2, 0},
		{3, 4, -1, 1},
		{7, 8, 9, 11, 12},
	}

	for _, input := range inputs {
		result := firstMissingPositive(input)
		fmt.Println(result)
	}
}
