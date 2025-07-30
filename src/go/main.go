package main

import "fmt"

func shiftGrid(grid [][]int, k int) [][]int {
	n := len(grid)
	m := len(grid[0])

	result := make([][]int, n)
	for i := range result {
		result[i] = make([]int, m)
	}

	for r := 0; r < n; r += 1 {
		for c := 0; c < m; c += 1 {
			new_r := (r + (c+k)/m) % n
			new_c := (c + k) % m

			result[new_r][new_c] = grid[r][c]
		}
	}

	return result
}

type input struct {
	grid [][]int
	k    int
}

func main() {
	inputs := []input{
		{
			grid: [][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}},
			k:    1,
		},
		{
			grid: [][]int{{3, 8, 1, 9}, {19, 7, 2, 5}, {4, 6, 11, 10}, {12, 0, 21, 13}},
			k:    4,
		},
		{
			grid: [][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}},
			k:    9,
		},
	}

	for _, input := range inputs {
		result := shiftGrid(input.grid, input.k)
		fmt.Println(result)
	}
}
