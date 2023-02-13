package main

import (
	"fmt"
)

func countOdds(low int, high int) int {
	if (low & 1) == 0 {
		low += 1
	}

	if low > high {
		return 0
	}

	return ((high - low) / 2) + 1
}

func main() {
	inputs := [][]int{
		{3, 7},
		{8, 10},
	}

	for _, input := range inputs {
		result := countOdds(input[0], input[1])
		fmt.Println(result)
	}
}
