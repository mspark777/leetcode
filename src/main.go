package main

import (
	"fmt"
)

func min(a, b int) int {
	if a < b {
		return a
	}

	return b
}

func max(a, b int) int {
	if a < b {
		return b
	}

	return a
}

func abs(a int) int {
	if a < 0 {
		return -a
	}

	return a
}

func computeArea(ax1 int, ay1 int, ax2 int, ay2 int, bx1 int, by1 int, bx2 int, by2 int) int {
	overX := min(ax2, bx2) - max(ax1, bx1)
	overY := min(ay2, by2) - max(ay1, by1)

	areaA := (ay2 - ay1) * (ax2 - ax1)
	areaB := (by2 - by1) * (bx2 - bx1)
	areaC := 0
	if (overX > 0) && (overY > 0) {
		areaC = overX * overY
	}

	return abs(areaA) + abs(areaB) - areaC
}

func main() {
	inputs := [][]int{
		{-3, 0, 3, 4, 0, -1, 9, 2},
		{-2, -2, 2, 2, -2, -2, 2, 2},
	}

	for _, input := range inputs {
		result := computeArea(
			input[0],
			input[1],
			input[2],
			input[3],
			input[4],
			input[5],
			input[6],
			input[7],
		)
		fmt.Println(result)
	}
}
