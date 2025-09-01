package main

import (
	"fmt"
)

func findCenter(edges [][]int) int {
	firstU := edges[0][0]
	firstV := edges[0][1]
	secondU := edges[1][0]
	secondV := edges[1][1]

	if firstU == secondU || firstU == secondV {
		return firstU
	}

	return firstV
}

type input struct {
	edges [][]int
}

func main() {
	inputs := []input{
		{
			edges: [][]int{{1, 2}, {2, 3}, {4, 2}},
		},
		{

			edges: [][]int{{1, 2}, {5, 1}, {1, 3}, {1, 4}},
		},
	}

	for _, input := range inputs {
		result := findCenter(input.edges)
		fmt.Println(result)
	}
}
