package main

import (
	"fmt"
)

func countGoodRectangles(rectangles [][]int) int {
	result := 0
	maxLen := 0

	for _, rectangle := range rectangles {
		k := min(rectangle[0], rectangle[1])
		if k > maxLen {
			maxLen = k
			result = 1
		} else if k == maxLen {
			result++
		}
	}

	return result
}

type input struct {
	rectangles [][]int
}

func main() {
	inputs := []input{
		{
			rectangles: [][]int{
				{5, 8},
				{3, 9},
				{5, 12},
				{16, 5},
			},
		},
		{
			rectangles: [][]int{
				{2, 3},
				{3, 7},
				{4, 3},
				{3, 7},
			},
		},
	}

	for _, input := range inputs {
		result := countGoodRectangles(input.rectangles)
		fmt.Println(result)
	}
}
