package main

import (
	"fmt"
	"math"
)

func maxSumSubmatrix(matrix [][]int, k int) int {
	rowCount := len(matrix)
	colCount := len(matrix[0])
	maxSum := math.MinInt

	for i0 := 0; i0 < colCount; i0 += 1 {
		sums := make([]int, rowCount)
		for i1 := i0; i1 < colCount; i1 += 1 {
			for i2 := 0; i2 < rowCount; i2 += 1 {
				sums[i2] += matrix[i2][i1]
			}

			for i2 := 0; i2 < rowCount; i2 += 1 {
				sum := 0
				for i3 := i2; i3 < rowCount; i3 += 1 {
					sum += sums[i3]
					if (sum > maxSum) && (sum <= k) {
						maxSum = sum
					}
				}
			}
		}
	}

	return maxSum
}

type input struct {
	matrix [][]int
	k      int
}

func main() {
	inputs := []input{
		{
			matrix: [][]int{{1, 0, 1}, {0, -2, 3}},
			k:      2,
		},
		{
			matrix: [][]int{{2, 2, -1}},
			k:      3,
		},
	}

	for _, input := range inputs {
		matrix := input.matrix
		k := input.k
		result := maxSumSubmatrix(matrix, k)
		fmt.Println(result)
	}
}
