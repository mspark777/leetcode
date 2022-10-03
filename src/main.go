package main

import (
	"fmt"
)

func minCost(colors string, neededTime []int) int {
	total := 0
	cur := neededTime[0]

	for i := 1; i < len(colors); i += 1 {
		if colors[i] != colors[i-1] {
			cur = 0
		}

		needed := neededTime[i]
		if cur < needed {
			total += cur
			cur = needed
		} else {
			total += needed
		}
	}

	return total
}

type input struct {
	colors     string
	neededTime []int
}

func main() {
	inputs := []input{
		{
			colors:     "abaac",
			neededTime: []int{1, 2, 3, 4, 5},
		},
		{
			colors:     "abc",
			neededTime: []int{1, 2, 3},
		},
		{
			colors:     "aabaa",
			neededTime: []int{1, 2, 3, 4, 1},
		},
	}

	for _, input := range inputs {
		colors := input.colors
		neededTime := input.neededTime
		result := minCost(colors, neededTime)
		fmt.Println(result)
	}
}
