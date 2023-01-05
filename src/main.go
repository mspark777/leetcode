package main

import (
	"fmt"
	"sort"
)

func findMinArrowShots(points [][]int) int {
	sort.Slice(points, func(i, j int) bool {
		return points[i][1] < points[j][1]
	})

	result := 1
	prev := 0

	for cur := 1; cur < len(points); cur += 1 {
		if points[cur][0] > points[prev][1] {
			result += 1
			prev = cur
		}
	}

	return result
}

func main() {
	inputs := [][][]int{
		{{10, 16}, {2, 8}, {1, 6}, {7, 12}},
		{{1, 2}, {3, 4}, {5, 6}, {7, 8}},
		{{1, 2}, {2, 3}, {3, 4}, {4, 5}},
	}

	for _, points := range inputs {
		result := findMinArrowShots(points)
		fmt.Println(result)
	}
}
