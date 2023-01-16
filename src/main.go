package main

import (
	"fmt"
	"sort"
)

func insert(intervals [][]int, newInterval []int) [][]int {
	pos := sort.Search(len(intervals), func(i int) bool {
		return intervals[i][0] > newInterval[0]
	})
	intervals = append(intervals, newInterval)
	if pos >= 0 {
		copy(intervals[pos+1:], intervals[pos:])
		intervals[pos] = newInterval
	}

	result := [][]int{intervals[0]}

	for i := 0; i < len(intervals); i += 1 {
		last := result[len(result)-1]
		interval := intervals[i]
		if interval[0] > last[1] {
			result = append(result, interval)
		} else {
			if interval[1] > last[1] {
				last[1] = interval[1]
			}
		}
	}

	return result
}

type input struct {
	intervals   [][]int
	newInterval []int
}

func main() {
	inputs := []input{
		{intervals: [][]int{{1, 3}, {6, 9}}, newInterval: []int{2, 5}},
		{intervals: [][]int{{1, 2}, {3, 5}, {6, 7}, {8, 10}, {12, 16}}, newInterval: []int{4, 8}},
		{intervals: [][]int{}, newInterval: []int{5, 7}},
	}

	for _, input := range inputs {
		result := insert(input.intervals, input.newInterval)
		fmt.Println(result)
	}
}
