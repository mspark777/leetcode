package main

import (
	"fmt"
)

func diagonalSum(mat [][]int) int {
	result := 0
	for left := 0; left < len(mat); left += 1 {
		right := len(mat) - (left + 1)
		result += mat[left][left] + mat[right][left]
	}

	if (len(mat) & 1) == 1 {
		m := len(mat) / 2
		result -= mat[m][m]
	}

	return result
}

func main() {
	inputs := [][][]int{
		{
			{1, 2, 3},
			{4, 5, 6},
			{7, 8, 9},
		},
		{
			{1, 1, 1, 1},
			{1, 1, 1, 1},
			{1, 1, 1, 1},
			{1, 1, 1, 1},
		},
		{
			{5},
		},
	}

	for _, mat := range inputs {
		result := diagonalSum(mat)
		fmt.Println(result)
	}
}
