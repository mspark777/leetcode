package main

import (
	"fmt"
)

const (
	LEFT = iota
	RIGHT
	UP
	DOWN
)

func spiralOrder(matrix [][]int) []int {
	rowCount := len(matrix)
	colCount := len(matrix[0])
	left := 0
	right := colCount - 1
	top := 0
	bottom := rowCount - 1
	dir := RIGHT
	i := 0
	result := make([]int, rowCount*colCount)

	for (left <= right) && (top <= bottom) {
		if dir == RIGHT {
			for col := left; col <= right; col += 1 {
				result[i] = matrix[top][col]
				i += 1
			}

			top += 1
			dir = DOWN
		} else if dir == DOWN {
			for row := top; row <= bottom; row += 1 {
				result[i] = matrix[row][right]
				i += 1
			}

			right -= 1
			dir = LEFT
		} else if dir == LEFT {
			for col := right; col >= left; col -= 1 {
				result[i] = matrix[bottom][col]
				i += 1
			}

			bottom -= 1
			dir = UP
		} else {
			for row := bottom; row >= top; row -= 1 {
				result[i] = matrix[row][left]
				i += 1
			}

			left += 1
			dir = RIGHT
		}
	}

	return result
}

func main() {
	inputs := [][][]int{
		{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}},
		{{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}},
	}

	for _, input := range inputs {
		result := spiralOrder(input)
		fmt.Println(result)
	}
}
