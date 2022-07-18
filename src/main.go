package main

import (
	"fmt"
	"leetcode/src/solution"
)

type input struct {
	matrix [][]int
	target int
}

func main() {
	inputs := []input{
		{
			matrix: [][]int{{0, 1, 0}, {1, 1, 1}, {0, 1, 0}},
			target: 0,
		},
		{
			matrix: [][]int{{1, -1}, {-1, 1}},
			target: 0,
		},
		{
			matrix: [][]int{{904}},
			target: 0,
		},
	}

	for _, input := range inputs {
		result := solution.NumSubmatrixSumTarget(input.matrix, input.target)
		fmt.Printf("%v\n", result)
	}
}
