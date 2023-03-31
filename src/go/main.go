package main

import (
	"fmt"
)

func btoi(b bool) int {
	if b {
		return 1
	}

	return 0
}

func matrix(rows, cols int) [][]int {
	matrix := make([][]int, rows)
	for i := 0; i < rows; i += 1 {
		matrix[i] = make([]int, cols)
	}

	return matrix
}

func ways(pizza []string, k int) int {
	rows := len(pizza)
	cols := len(pizza[0])
	apples := matrix(rows+1, cols+1)
	f := matrix(rows, cols)

	for row := rows - 1; row >= 0; row -= 1 {
		for col := cols - 1; col >= 0; col -= 1 {
			apples[row][col] = btoi(pizza[row][col] == 'A') +
				apples[row+1][col] +
				apples[row][col+1] -
				apples[row+1][col+1]
			f[row][col] = btoi(apples[row][col] > 0)
		}
	}

	const MOD = 1000000007
	for remain := 1; remain < k; remain += 1 {
		g := matrix(rows, cols)
		for row := 0; row < rows; row += 1 {
			for col := 0; col < cols; col += 1 {
				for nextRow := row + 1; nextRow < rows; nextRow += 1 {
					if (apples[row][col] - apples[nextRow][col]) > 0 {
						g[row][col] += f[nextRow][col]
						g[row][col] %= MOD
					}
				}

				for nextCol := col + 1; nextCol < cols; nextCol += 1 {
					if (apples[row][col] - apples[row][nextCol]) > 0 {
						g[row][col] += f[row][nextCol]
						g[row][col] %= MOD
					}
				}
			}
		}
		f = g
	}
	return f[0][0]

}

type input struct {
	pizza []string
	k     int
}

func main() {
	inputs := []input{
		{[]string{"A..", "AAA", "..."}, 3},
		{[]string{"A..", "AA.", "..."}, 3},
		{[]string{"A..", "A..", "..."}, 1},
	}

	for _, input := range inputs {
		result := ways(input.pizza, input.k)
		fmt.Println(result)
	}
}
