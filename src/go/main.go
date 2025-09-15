package main

import (
	"fmt"
)

func findMiddleIndex(nums []int) int {
	right := 0
	for _, num := range nums {
		right += num
	}

	left := 0
	for i, num := range nums {
		right -= num
		if left == right {
			return i
		}

		left += num
	}

	return -1
}

type input struct {
	nums []int
}

func main() {
	inputs := []input{
		{
			nums: []int{2, 3, -1, 8, 4},
		},
		{
			nums: []int{1, -1, 4},
		},
		{
			nums: []int{2, 5},
		},
	}

	for _, input := range inputs {
		result := findMiddleIndex(input.nums)
		fmt.Println(result)
	}
}
