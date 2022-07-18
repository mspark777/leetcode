package main

import (
	"fmt"
)

type input struct {
	prices []int
}

func main() {
	inputs := []input{
		{
			prices: []int{7, 1, 5, 3, 6, 4},
		},
		{
			prices: []int{7, 6, 4, 3, 1},
		},
		{
			prices: []int{3, 3, 5, 0, 0, 3, 1, 4},
		},
	}

	for _, input := range inputs {
		result := maxProfit(input.prices)
		fmt.Printf("%v\n", result)
	}
}
