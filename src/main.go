package main

import (
	"fmt"
	"sort"
)

func eraseOverlapIntervals(intervals [][]int) int {
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][1] < intervals[j][1]
	})

	result := 0
	k := -(0x8FFFFFFF)
	for _, interval := range intervals {
		x := interval[0]
		y := interval[1]

		if x >= k {
			k = y
		} else {
			result += 1
		}
	}

	return result
}

func main() {
	inputs := [][][]int{
		{{1, 2}, {2, 3}, {3, 4}, {1, 3}},
		{{1, 2}, {1, 2}, {1, 2}},
		{{1, 2}, {2, 3}},
	}

	for _, input := range inputs {
		result := eraseOverlapIntervals(input)
		fmt.Println(result)
	}
}
