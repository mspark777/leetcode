package main

func generate(numRows int) [][]int {
	result := make([][]int, numRows)
	for i := 0; i < numRows; i++ {
		row := make([]int, i+1)
		row[0] = 1
		row[i] = 1
		prev := i - 1
		for j := 1; j < i; j++ {
			row[j] = result[prev][j-1] + result[prev][j]
		}

		result[i] = row
	}

	return result
}
