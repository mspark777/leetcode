package main

import (
	"fmt"
)

func updateMatrix(mat [][]int) [][]int {
	rowCount := len(mat)
	colCount := len(mat[0])
	result := make([][]int, rowCount)
	for r := 0; r < rowCount; r += 1 {
		result[r] = make([]int, colCount)
	}

	queue := [][]int{}
	maxValue := rowCount * colCount

	for r := 0; r < rowCount; r += 1 {
		for c := 0; c < colCount; c += 1 {
			if mat[r][c] == 0 {
				queue = append(queue, []int{r, c})
			} else {
				result[r][c] = maxValue
			}
		}
	}

	directions := [][]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for len(queue) > 0 {
		head := queue[0]
		queue = queue[1:]

		row := head[0]
		col := head[1]
		cell0 := result[row][col] + 1

		for _, dir := range directions {
			dr := dir[0]
			dc := dir[1]
			r := row + dr
			c := col + dc
			if r < 0 {
				continue
			} else if r >= rowCount {
				continue
			} else if c < 0 {
				continue
			} else if c >= colCount {
				continue
			}

			cell1 := result[r][c]
			if cell1 <= cell0 {
				continue
			}

			queue = append(queue, []int{r, c})
			result[r][c] = cell0
		}
	}

	return result
}

func main() {
	inputs := [][][]int{
		{{0, 0, 0}, {0, 1, 0}, {0, 0, 0}},
		{{0, 0, 0}, {0, 1, 0}, {1, 1, 1}},
	}

	for _, mat := range inputs {
		result := updateMatrix(mat)
		fmt.Println(result)
	}
}
