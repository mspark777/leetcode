package main

import (
	"fmt"
)

type solution struct {
	results      [][]int
	combinations []int
}

func (solution *solution) backtrack(start, n, k int) {
	if len(solution.combinations) == k {
		temp := make([]int, len(solution.combinations))
		copy(temp, solution.combinations)
		solution.results = append(solution.results, temp)
		return
	}

	for i := start; i <= n; i += 1 {
		solution.combinations = append(solution.combinations, i)
		solution.backtrack(i+1, n, k)
		last := len(solution.combinations) - 1
		solution.combinations = solution.combinations[0:last]
	}
}

func combine(n int, k int) [][]int {
	solution := solution{
		results:      [][]int{},
		combinations: []int{},
	}
	solution.backtrack(1, n, k)
	return solution.results
}

type input struct {
	n int
	k int
}

func main() {
	inputs := []input{
		{4, 2},
		{1, 1},
	}

	for _, input := range inputs {
		result := combine(input.n, input.k)
		fmt.Println(result)
	}
}
