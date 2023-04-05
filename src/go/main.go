package main

import (
	"fmt"
)

func minimizeArrayValue(nums []int) int {
	result := 0
	prefix_sum := 0

	for i, num := range nums {
		prefix_sum += num
		sum := (prefix_sum + i) / (i + 1)
		if sum > result {
			result = sum
		}
	}

	return result
}

func main() {
	inputs := [][]int{
		{3, 7, 1, 6},
		{10, 1},
	}

	for _, input := range inputs {
		result := minimizeArrayValue(input)
		fmt.Println(result)
	}
}
