package main

import (
	"fmt"
)

func kthSmallest(matrix [][]int, k int) int {
	mlen := len(matrix)
	left := matrix[0][0]
	right := matrix[mlen-1][mlen-1]

	for left < right {
		mid := left + (right-left)/2
		count := 0

		for i := 0; i < mlen; i += 1 {
			for j := mlen - 1; j >= 0; j -= 1 {
				if matrix[i][j] <= mid {
					count += j + 1
					break
				}
			}
		}

		if count < k {
			left = mid + 1
		} else {
			right = mid
		}
	}

	return left
}

type input struct {
	matrix [][]int
	k      int
}

func main() {
	inputs := []*input{
		{
			matrix: [][]int{{1, 5, 9}, {10, 11, 13}, {12, 13, 15}},
			k:      8,
		},
		{
			matrix: [][]int{{-5}},
			k:      1,
		},
		{
			matrix: [][]int{{-5, -4}, {-5, -4}},
			k:      2,
		},
		{
			matrix: [][]int{{1, 2}, {1, 3}},
			k:      1,
		},
	}

	for _, input := range inputs {
		matrix := input.matrix
		k := input.k
		result := kthSmallest(matrix, k)
		fmt.Println(result)
	}
}
