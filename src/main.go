package main

import (
	"fmt"
)

func combinationSum4(nums []int, target int) int {
	result := make([]int, target+1)
	result[0] = 1
	for i := 1; i <= target; i += 1 {
		for _, num := range nums {
			if i >= num {
				result[i] += result[i-num]
			}
		}
	}

	return result[target]
}

type input struct {
	nums   []int
	target int
}

func main() {
	inputs := []*input{
		{
			nums:   []int{1, 2, 3},
			target: 4,
		},
		{
			nums:   []int{9},
			target: 3,
		},
	}

	for _, input := range inputs {
		nums := input.nums
		target := input.target
		result := combinationSum4(nums, target)
		fmt.Println(result)
	}
}
