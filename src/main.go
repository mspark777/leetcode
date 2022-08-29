package main

import (
	"fmt"
)

const WATER = '0'
const LAND = '1'

func clearLand(grid [][]byte, row, col, rowCount, colCount int) {
	stack := [][]int{{row, col}}

	for len(stack) > 0 {
		idx := len(stack) - 1
		top := stack[idx]
		stack = stack[:idx]

		r := top[0]
		c := top[1]

		if (r < 0) || (c < 0) {
			continue
		} else if (r >= rowCount) || (c >= colCount) {
			continue
		} else if grid[r][c] == WATER {
			continue
		}

		grid[r][c] = WATER
		stack = append(stack,
			[]int{r - 1, c},
			[]int{r + 1, c},
			[]int{r, c - 1},
			[]int{r, c + 1},
		)
	}
}

func numIslands(grid [][]byte) int {
	rowCount := len(grid)
	colCount := len(grid[0])

	result := 0
	for r := 0; r < rowCount; r += 1 {
		for c := 0; c < colCount; c += 1 {
			if grid[r][c] == LAND {
				result += 1
				clearLand(grid, r, c, rowCount, colCount)
			}
		}
	}

	return result
}

type input struct {
	grid [][]byte
}

func main() {
	inputs := []input{
		{
			grid: [][]byte{
				{'1', '1', '1', '1', '0'},
				{'1', '1', '0', '1', '0'},
				{'1', '1', '0', '0', '0'},
				{'0', '0', '0', '0', '0'},
			},
		},
		{
			grid: [][]byte{
				{'1', '1', '0', '0', '0'},
				{'1', '1', '0', '0', '0'},
				{'0', '0', '1', '0', '0'},
				{'0', '0', '0', '1', '1'},
			},
		},
	}

	for _, input := range inputs {
		grid := input.grid
		result := numIslands(grid)
		fmt.Println(result)
	}
}
