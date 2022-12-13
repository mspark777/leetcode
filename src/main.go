package main

import (
	"fmt"
)

func getMin(i, j int) int {
	if i < j {
		return i
	}

	return j
}

func minFallingPathSum(matrix [][]int) int {
	rowCount := len(matrix)
	colCount := len(matrix[0])
	lastRowIdx := rowCount - 1
	lastColIdx := colCount - 1

	dp := make([][]int, rowCount)
	for i := 0; i < rowCount; i += 1 {
		dp[i] = make([]int, colCount)
	}

	for i := 0; i < colCount; i += 1 {
		dp[lastRowIdx][i] = matrix[lastRowIdx][i]
	}

	for i := rowCount - 2; i >= 0; i -= 1 {
		for j := 0; j < colCount; j += 1 {
			next := i + 1
			min := 2147483647
			if j < lastColIdx {
				min = getMin(dp[next][j+1], min)
			}

			if j > 0 {
				min = getMin(dp[next][j-1], min)
			}

			min = getMin(dp[next][j], min)
			dp[i][j] = matrix[i][j] + min
		}
	}

	min := 2147483647
	for i := 0; i < colCount; i += 1 {
		min = getMin(dp[0][i], min)
	}

	return min
}

func main() {
	inputs := [][][]int{
		{{2, 1, 3}, {6, 5, 4}, {7, 8, 9}},
		{{-19, 57}, {-40, -5}},
	}

	for _, matrix := range inputs {
		result := minFallingPathSum(matrix)
		fmt.Println(result)
	}
}
