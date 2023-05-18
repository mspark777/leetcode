package main

import (
	"fmt"
)

func findSmallestSetOfVertices(n int, edges [][]int) []int {
	toEdges := make([]bool, n)
	for _, edge := range edges {
		to := edge[1]
		toEdges[to] = true
	}

	result := []int{}
	for i, isto := range toEdges {
		if !isto {
			result = append(result, i)
		}
	}

	return result
}

type input struct {
	n     int
	edges [][]int
}

func main() {
	inputs := []input{
		{n: 6, edges: [][]int{{0, 1}, {0, 2}, {2, 5}, {3, 4}, {4, 2}}},
		{n: 5, edges: [][]int{{0, 1}, {2, 1}, {3, 1}, {1, 4}, {2, 4}}},
	}

	for _, input := range inputs {
		result := findSmallestSetOfVertices(input.n, input.edges)
		fmt.Println(result)
	}
}
