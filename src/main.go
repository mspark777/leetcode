package main

import (
	"fmt"
)

func contain(nums []int, num int) bool {
	for _, n := range nums {
		if n == num {
			return true
		}
	}

	return false
}

type solution struct {
	results      [][]int
	permutations []int
}

func (solution *solution) backtrack(nums []int) {
	if len(solution.permutations) == len(nums) {
		temp := make([]int, len(nums))
		copy(temp, solution.permutations)
		solution.results = append(solution.results, temp)
		return
	}

	for _, num := range nums {
		if contain(solution.permutations, num) {
			continue
		}

		solution.permutations = append(solution.permutations, num)
		solution.backtrack(nums)
		last := len(solution.permutations) - 1
		solution.permutations = solution.permutations[0:last]
	}
}

func permute(nums []int) [][]int {
	solution := solution{
		results:      [][]int{},
		permutations: []int{},
	}

	solution.backtrack(nums)
	return solution.results
}

func main() {
	inputs := [][]int{
		{1, 2, 3},
		{0, 1},
		{1},
	}

	for _, nums := range inputs {
		result := permute(nums)
		fmt.Println(result)
	}
}
