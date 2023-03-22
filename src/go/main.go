package main

import (
	"fmt"
)

func minScore(n int, roads [][]int) int {
	uf := UnionFindConstructor(n + 1)
	result := 0xffffffff

	for _, road := range roads {
		uf.Union(road[0], road[1])
	}

	for _, road := range roads {
		a := road[0]
		d := road[2]
		if uf.Find(1) == uf.Find(a) {
			if d < result {
				result = d
			}
		}
	}

	return result
}

type input struct {
	n     int
	roads [][]int
}

func main() {
	inputs := []input{
		{n: 4, roads: [][]int{{1, 2, 9}, {2, 3, 6}, {2, 4, 5}, {1, 4, 7}}},
		{n: 4, roads: [][]int{{1, 2, 2}, {1, 3, 4}, {3, 4, 7}}},
	}

	for _, input := range inputs {
		result := minScore(input.n, input.roads)
		fmt.Println(result)
	}
}
