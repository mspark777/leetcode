package main

import (
	"fmt"
	"sort"
)

func maxIceCream(costs []int, coins int) int {
	sort.Ints(costs)

	result := 0

	for _, cost := range costs {
		if coins >= cost {
			coins -= cost
			result += 1
		} else {
			break
		}
	}

	return result
}

type input struct {
	costs []int
	coins int
}

func main() {
	inputs := []input{
		{costs: []int{1, 3, 2, 4, 1}, coins: 7},
		{costs: []int{10, 6, 8, 7, 7, 8}, coins: 5},
		{costs: []int{1, 6, 3, 1, 2, 5}, coins: 20},
	}

	for _, input := range inputs {
		result := maxIceCream(input.costs, input.coins)
		fmt.Println(result)
	}
}
