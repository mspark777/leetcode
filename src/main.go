package main

import (
	"fmt"
)

type input struct {
	nums   []int
	target int
}

func main() {
	inputs := []input{
		{
			nums: []int{5, 7, 7, 8, 8, 10}, target: 8,
		},
		{
			nums: []int{5, 7, 7, 8, 8, 10}, target: 6,
		},
		{
			nums: []int{}, target: 0,
		},
	}

	for _, input := range inputs {
		result := searchRange(input.nums, input.target)
		fmt.Printf("%v\n", result)
	}
}
