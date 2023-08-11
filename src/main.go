package main

import (
	"fmt"
	"sort"
)

func merge(intervals [][]int) [][]int {
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][0] < intervals[j][0]
	})

	result := [][]int{intervals[0]}
	for _, interval := range intervals[1:] {
		last := result[len(result)-1]
		lastEnd := last[1]
		start := interval[0]
		if lastEnd < start {
			result = append(result, interval)
			continue
		}

		end := interval[1]
		if end > lastEnd {
			last[1] = end
		}
	}

	return result
}

func main() {
	inputs := [][][]int{
		{{1, 3}, {2, 6}, {8, 10}, {15, 18}},
		{{1, 4}, {4, 5}},
	}

	for _, input := range inputs {
		result := merge(input)
		fmt.Println(result)
	}
}
