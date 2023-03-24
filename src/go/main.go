package main

import (
	"fmt"
)

func dfs(node, parent int, adjs map[int][][]int, count *int) {
	edges, ok := adjs[node]
	if !ok {
		return
	}

	for _, edge := range edges {
		child := edge[0]
		sign := edge[1]

		if child != parent {
			*count += sign
			dfs(child, node, adjs, count)
		}
	}
}

func minReorder(n int, connections [][]int) int {
	adjs := make(map[int][][]int, n)
	for _, conn := range connections {
		a := conn[0]
		b := conn[1]

		aedges, aok := adjs[a]
		bedges, bok := adjs[b]

		if !aok {
			aedges = [][]int{}
		}

		if !bok {
			bedges = [][]int{}
		}

		aedges = append(aedges, []int{b, 1})
		bedges = append(bedges, []int{a, 0})

		adjs[a] = aedges
		adjs[b] = bedges
	}

	result := 0
	dfs(0, -1, adjs, &result)

	return result
}

type input struct {
	n           int
	connections [][]int
}

func main() {
	inputs := []input{
		{n: 6, connections: [][]int{{0, 1}, {1, 3}, {2, 3}, {4, 0}, {4, 5}}},
		{n: 5, connections: [][]int{{1, 0}, {1, 2}, {3, 2}, {3, 4}}},
		{n: 0, connections: [][]int{{1, 0}, {2, 0}}},
	}

	for _, input := range inputs {
		result := minReorder(input.n, input.connections)
		fmt.Println(result)
	}
}
