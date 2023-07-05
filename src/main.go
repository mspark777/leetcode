package main

import (
	"fmt"
)

func longestSubarray(nums []int) int {
	zeroCount := 0
	result := 0
	start := 0

	for i, num := range nums {
		if num == 0 {
			zeroCount += 1
		}

		for zeroCount > 1 {
			if nums[start] == 0 {
				zeroCount -= 1
			}

			start += 1
		}

		oneCount := i - start
		if oneCount > result {
			result = oneCount
		}
	}

	return result
}

func main() {
	inputs := [][]int{
		{1, 1, 0, 1},
		{0, 1, 1, 1, 0, 1, 1, 0, 1},
		{1, 1, 1},
	}

	for _, input := range inputs {
		result := longestSubarray(input)
		fmt.Println(result)
	}
}
