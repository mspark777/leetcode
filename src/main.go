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
			nums: []int{5, 2, 6, 1},
		},
		{
			nums: []int{-1},
		},
		{
			nums: []int{-1, -1},
		},
	}

	for _, input := range inputs {
		result := countSmaller(input.nums)
		fmt.Printf("%v\n", result)
	}
}
