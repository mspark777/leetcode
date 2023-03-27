package main

import (
	"fmt"
)

func minPathSum(grid [][]int) int {
	row := len(grid)
	col := len(grid[0])
	lastRow := row - 1
	lastCol := col - 1
	const MAX = 1024 * 1024 * 1024 * 2

	for r := lastRow; r >= 0; r -= 1 {
		for c := lastCol; c >= 0; c -= 1 {
			if (r == lastRow) && (c == lastCol) {
				continue
			}

			rightMin := MAX
			downMin := MAX
			if c < lastCol {
				rightMin = grid[r][c+1]
			}

			if r < lastRow {
				downMin = grid[r+1][c]
			}

			if rightMin < downMin {
				grid[r][c] += rightMin
			} else {
				grid[r][c] += downMin
			}
		}
	}

	return grid[0][0]
}

func main() {
	inputs := [][][]int{
		{{1, 3, 1}, {1, 5, 1}, {4, 2, 1}},
		{{1, 2, 3}, {4, 5, 6}},
	}

	for _, input := range inputs {
		result := minPathSum(input)
		fmt.Println(result)
	}
}
