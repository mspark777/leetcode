package main

import (
	"fmt"
)

func kidsWithCandies(candies []int, extraCandies int) []bool {
	maxCandy := -1
	for _, candy := range candies {
		if candy > maxCandy {
			maxCandy = candy
		}
	}

	maxCandy -= extraCandies

	result := make([]bool, len(candies))
	for i, candy := range candies {
		result[i] = candy >= maxCandy
	}

	return result
}

type input struct {
	candies      []int
	extraCandies int
}

func main() {
	inputs := []input{
		{[]int{2, 3, 5, 1, 3}, 3},
		{[]int{4, 2, 1, 1, 2}, 1},
		{[]int{12, 1, 12}, 10},
	}

	for _, input := range inputs {
		result := kidsWithCandies(input.candies, input.extraCandies)
		fmt.Println(result)
	}
}
