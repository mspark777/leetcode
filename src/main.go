package main

import (
	"fmt"
)

func transpose(matrix [][]int) {
	for i := 0; i < len(matrix); i += 1 {
		for j := i + 1; j < len(matrix); j += 1 {
			temp := matrix[i][j]
			matrix[i][j] = matrix[j][i]
			matrix[j][i] = temp
		}
	}
}

func reverse(matrix [][]int) {
	for i := 0; i < len(matrix); i += 1 {
		j := 0
		k := len(matrix) - 1
		for j < k {
			temp := matrix[i][j]
			matrix[i][j] = matrix[i][k]
			matrix[i][k] = temp
			j += 1
			k -= 1
		}
	}
}

func rotate(matrix [][]int) {
	transpose(matrix)
	reverse(matrix)
}

type input struct {
	matrix [][]int
}

func main() {
	inputs := []input{
		{
			matrix: [][]int{
				{1, 2, 3},
				{4, 5, 6},
				{7, 8, 9},
			},
		},
		{
			matrix: [][]int{
				{5, 1, 9, 11}, {2, 4, 8, 10}, {13, 3, 6, 7}, {15, 14, 12, 16},
			},
		},
	}

	for _, input := range inputs {
		matrix := input.matrix
		rotate(matrix)
		fmt.Println(matrix)
	}
}
