package main

import (
	"fmt"
)

type input struct {
	nums []int
}

func main() {
	inputs := []*input{
		{
			nums: []int{1, 3, 5},
		},
	}

	for _, input := range inputs {
		narr := Constructor(input.nums)
		fmt.Printf("%v\n", narr.SumRange(0, 2))
		narr.Update(1, 2)
		fmt.Printf("%v\n", narr.SumRange(0, 2))
	}
}
