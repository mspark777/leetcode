package main

import (
	"fmt"
)

func shuffle(nums []int, n int) []int {
	result := make([]int, len(nums))

	for i := 0; i < n; i += 1 {
		j := i * 2
		result[j] = nums[i]
		result[j+1] = nums[n+i]
	}

	return result
}

type input struct {
	nums []int
	n    int
}

func main() {
	inputs := []input{
		{nums: []int{2, 5, 1, 3, 4, 7}, n: 3},
		{nums: []int{1, 2, 3, 4, 4, 3, 2, 1}, n: 4},
	}

	for _, input := range inputs {
		result := shuffle(input.nums, input.n)
		fmt.Println(result)
	}
}
