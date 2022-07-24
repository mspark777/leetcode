package main

import (
	"fmt"
)

type input struct {
	matrix [][]int
	target int
}

func main() {
	inputs := []input{
		{
			matrix: [][]int{{1, 4, 7, 11, 15}, {2, 5, 8, 12, 19}, {3, 6, 9, 16, 22}, {10, 13, 14, 17, 24}, {18, 21, 23, 26, 30}},
			target: 5,
		},
		{
			matrix: [][]int{{1, 4, 7, 11, 15}, {2, 5, 8, 12, 19}, {3, 6, 9, 16, 22}, {10, 13, 14, 17, 24}, {18, 21, 23, 26, 30}},
			target: 20,
		},
	}

	for _, input := range inputs {
		result := searchMatrix(input.matrix, input.target)
		fmt.Printf("%v\n", result)
	}
}
