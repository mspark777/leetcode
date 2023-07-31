package main

import (
	"fmt"
	"sort"
)

type solution struct {
	results      [][]int
	combinations []int
}

func (solution *solution) backtrack(candidates []int, remain, cur int) {
	if remain == 0 {
		temp := make([]int, len(solution.combinations))
		copy(temp, solution.combinations)
		solution.results = append(solution.results, temp)
		return
	}

	for next := cur; next < len(candidates); next += 1 {
		if (next > cur) && (candidates[next] == candidates[next-1]) {
			continue
		}

		pick := candidates[next]
		nextRemain := remain - pick
		if nextRemain < 0 {
			break
		}

		solution.combinations = append(solution.combinations, pick)
		solution.backtrack(candidates, nextRemain, next+1)
		top := len(solution.combinations) - 1
		solution.combinations = solution.combinations[0:top]
	}
}

func combinationSum2(candidates []int, target int) [][]int {
	solution := solution{
		results:      [][]int{},
		combinations: []int{},
	}
	sort.Ints(candidates)
	solution.backtrack(candidates, target, 0)

	return solution.results
}

type input struct {
	candidates []int
	target     int
}

func main() {
	inputs := []input{
		{candidates: []int{10, 1, 2, 7, 6, 1, 5}, target: 8},
		{candidates: []int{2, 5, 2, 1, 2}, target: 5},
	}

	for _, input := range inputs {
		result := combinationSum2(input.candidates, input.target)
		fmt.Println(result)
	}
}
