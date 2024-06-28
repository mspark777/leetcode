package main

import (
	"fmt"
	"sort"
)

func maximumImportance(n int, roads [][]int) int64 {
	degree := make([]int64, n)

	for _, edge := range roads {
		degree[edge[0]] += 1
		degree[edge[1]] += 1
	}

	sort.Slice(degree, func(i, j int) bool {
		return degree[i] < degree[j]
	})

	value := int64(1)
	result := int64(0)

	for _, d := range degree {
		result += value * d
		value += 1
	}

	return result
}

type input struct {
	roads [][]int
	n     int
}

func main() {
	inputs := []input{
		{
			[][]int{{0, 1}, {1, 2}, {2, 3}, {0, 2}, {1, 3}, {2, 4}}, 5,
		},
		{
			[][]int{{0, 3}, {2, 4}, {1, 3}}, 5,
		},
	}

	for _, input := range inputs {
		result := maximumImportance(input.n, input.roads)
		fmt.Println(result)
	}
}
