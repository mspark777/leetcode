package main

import (
	"fmt"
)

func isToeplitzMatrix(matrix [][]int) bool {
	for r := 1; r < len(matrix); r += 1 {
		for c := 1; c < len(matrix[r]); c += 1 {
			if matrix[r-1][c-1] != matrix[r][c] {
				return false
			}
		}
	}

	return true
}

func main() {
	inputs := [][][]int{
		{{1, 2, 3, 4}, {5, 1, 2, 3}, {9, 5, 1, 2}},
		{{1, 2}, {2, 2}},
	}

	for _, matrix := range inputs {
		result := isToeplitzMatrix(matrix)
		fmt.Println(result)
	}
}
