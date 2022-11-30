package main

import (
	"fmt"
)

func uniqueOccurrences(arr []int) bool {
	counts := map[int]int{}
	for _, n := range arr {
		counts[n] += 1
	}

	occurrences := map[int]bool{}
	for _, c := range counts {
		occurrences[c] = true
	}

	return len(counts) == len(occurrences)
}

func main() {
	inputs := [][]int{
		{1, 2, 2, 1, 1, 3},
		{1, 2},
		{-3, 0, 1, -3, 1, 1, 1, -3, 10, 0},
	}

	for _, arr := range inputs {
		result := uniqueOccurrences(arr)
		fmt.Println(result)
	}
}
