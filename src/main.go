package main

import (
	"fmt"
	"sort"
)

func diagonalSort(mat [][]int) [][]int {
	rowCount := len(mat)
	colCount := len(mat[0])

	queues := make(map[int][]int)
	for i := 0; i < rowCount; i += 1 {
		for j := 0; j < colCount; j += 1 {
			key := i - j
			value := mat[i][j]
			if queue, ok := queues[key]; ok {
				queues[key] = append(queue, value)
			} else {
				queues[key] = []int{value}
			}
		}
	}

	for _, queue := range queues {
		sort.Sort(sort.Reverse(sort.IntSlice(queue)))
	}

	result := make([][]int, rowCount)
	for i := 0; i < rowCount; i += 1 {
		row := make([]int, colCount)
		for j := 0; j < colCount; j += 1 {
			key := i - j
			queue := queues[key]
			top := len(queue) - 1
			row[j] = queue[top]
			queues[key] = queue[:top]
		}
		result[i] = row
	}

	return result
}

type input struct {
	mat [][]int
}

func main() {
	inputs := []input{
		{
			mat: [][]int{{3, 3, 1, 1}, {2, 2, 1, 2}, {1, 1, 1, 2}},
		},
		{
			mat: [][]int{{11, 25, 66, 1, 69, 7}, {23, 55, 17, 45, 15, 52}, {75, 31, 36, 44, 58, 8}, {22, 27, 33, 25, 68, 4}, {84, 28, 14, 11, 5, 50}},
		},
	}

	for _, input := range inputs {
		mat := input.mat
		result := diagonalSort(mat)
		fmt.Println(result)
	}
}
