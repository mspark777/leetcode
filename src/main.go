package main

import (
	"fmt"
)

type input struct {
	nums []int
}

func main() {
	inputs := []input{
		{
			nums: []int{1, 2, 3, 1},
		},
		{
			nums: []int{1, 2, 3, 4},
		},
		{
			nums: []int{1, 1, 1, 3, 3, 4, 3, 2, 4, 2},
		},
	}

	for _, input := range inputs {
		result := containsDuplicate(input.nums)
		fmt.Printf("%v\n", result)
	}
}
