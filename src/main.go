package main

import (
	"fmt"
)

func countNegatives(grid [][]int) int {
	result := 0
	colCount := len(grid[0])
	cur := colCount - 1

	for _, row := range grid {
		for cur >= 0 {
			if row[cur] < 0 {
				cur -= 1
			} else {
				break
			}
		}

		result += colCount - (cur + 1)
	}

	return result
}

func main() {
	inputs := [][][]int{
		{{4, 3, 2, -1}, {3, 2, 1, -1}, {1, 1, -1, -2}, {-1, -1, -2, -3}},
		{{3, 2}, {1, 0}},
	}

	for _, grid := range inputs {
		result := countNegatives(grid)
		fmt.Println(result)
	}
}
