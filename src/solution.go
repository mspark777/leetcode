package main

func searchMatrix(matrix [][]int, target int) bool {
	row := len(matrix) - 1
	col := 0
	countcol := len(matrix[0])

	for (row >= 0) && (col < countcol) {
		n := matrix[row][col]

		if target > n {
			col += 1
		} else if target < n {
			row -= 1
		} else {
			return true
		}
	}

	return false
}
