package main

import (
	"fmt"
	"sort"
)

func maxSatisfaction(satisfaction []int) int {
	sort.Slice(satisfaction, func(i, j int) bool {
		return satisfaction[j] < satisfaction[i]
	})

	result := 0
	suffix := 0

	for _, s := range satisfaction {
		suffix += s
		if suffix <= 0 {
			break
		}

		result += suffix
	}

	return result
}

func main() {
	inputs := [][]int{
		{-1, -8, 0, 5, -9},
		{4, 3, 2},
		{-1, -4, -5},
	}

	for _, input := range inputs {
		result := maxSatisfaction(input)
		fmt.Println(result)
	}
}
