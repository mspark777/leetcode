package main

import (
	"fmt"
)

func mostVisited(n int, rounds []int) []int {
	start := rounds[0]
	end := rounds[len(rounds)-1]
	result := []int{}

	if start <= end {
		for i := start; i <= end; i += 1 {
			result = append(result, i)
		}
	} else {
		for i := 1; i <= end; i += 1 {
			result = append(result, i)
		}

		for i := start; i <= n; i += 1 {
			result = append(result, i)
		}
	}

	return result
}

type input struct {
	n      int
	rounds []int
}

func main() {
	inputs := []input{
		{
			n:      4,
			rounds: []int{1, 3, 1, 2},
		},
		{
			n:      2,
			rounds: []int{2, 1, 2, 1, 2, 1, 2, 1, 2},
		},
		{
			n:      7,
			rounds: []int{1, 3, 5, 7},
		},
	}

	for _, input := range inputs {
		result := mostVisited(input.n, input.rounds)
		fmt.Println(result)
	}
}
