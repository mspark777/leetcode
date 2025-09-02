package main

import (
	"fmt"
)

func minOperations(nums []int) int {
	result := 0
	last := 0
	for _, num := range nums {
		result += max(0, last-num+1)
		last = max(num, last+1)
	}

	return result
}

type input struct {
	nums []int
}

func main() {
	inputs := []input{
		{
			nums: []int{1, 1, 1},
		},
		{
			nums: []int{1, 5, 2, 4, 1},
		},
		{
			nums: []int{8},
		},
	}

	for _, input := range inputs {
		result := minOperations(input.nums)
		fmt.Println(result)
	}
}
