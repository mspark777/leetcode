package main

import (
	"fmt"
)

func maximalNetworkRank(n int, roads [][]int) int {
	adjacents := map[int]map[int]bool{}

	for _, road := range roads {
		a := road[0]
		b := road[1]
		if set, ok := adjacents[a]; ok {
			set[b] = true
		} else {
			set := map[int]bool{}
			set[b] = true
			adjacents[a] = set
		}

		if set, ok := adjacents[b]; ok {
			set[a] = true
		} else {
			set := map[int]bool{}
			set[a] = true
			adjacents[b] = set
		}
	}

	result := 0
	for node0 := 0; node0 < n; node0 += 1 {
		set0, ok := adjacents[node0]
		if !ok {
			set0 = map[int]bool{}
		}

		rank0 := len(set0)
		for node1 := node0 + 1; node1 < n; node1 += 1 {
			set1, ok := adjacents[node1]
			rank1 := 0
			if ok {
				rank1 = len(set1)
			}

			rank := rank0 + rank1
			if _, ok := set0[node1]; ok {
				rank -= 1
			}

			if result < rank {
				result = rank
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
		{n: 4, roads: [][]int{{0, 1}, {0, 3}, {1, 2}, {1, 3}}},
		{n: 5, roads: [][]int{{0, 1}, {0, 3}, {1, 2}, {1, 3}, {2, 3}, {2, 4}}},
		{n: 8, roads: [][]int{{0, 1}, {1, 2}, {2, 3}, {2, 4}, {5, 6}, {5, 7}}},
	}

	for _, input := range inputs {
		result := maximalNetworkRank(input.n, input.roads)
		fmt.Println(result)
	}
}
