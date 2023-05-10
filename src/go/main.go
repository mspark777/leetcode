package main

import (
	"fmt"
)

func floorMod(x, y int) int {
	return ((x % y) + y) % y
}

func generateMatrix(n int) [][]int {
	result := make([][]int, n)
	cnt := 1
	dir := [][]int{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}
	d := 0
	row := 0
	col := 0

	for i := 0; i < n; i += 1 {
		result[i] = make([]int, n)
	}

	for cnt <= (n * n) {
		result[row][col] = cnt
		cnt += 1
		r := floorMod(row+dir[d][0], n)
		c := floorMod(col+dir[d][1], n)

		if result[r][c] != 0 {
			d = (d + 1) % 4
		}

		row += dir[d][0]
		col += dir[d][1]
	}

	return result
}

func main() {
	inputs := []int{
		3, 1,
	}

	for _, n := range inputs {
		result := generateMatrix(n)
		fmt.Println(result)
	}
}
