package main

import (
	"fmt"
)

func makeConnected(n int, connections [][]int) int {
	if len(connections) < (n - 1) {
		return -1
	}

	uf := UnionFindConstructor(n)
	result := n

	for _, conn := range connections {
		a := conn[0]
		b := conn[1]

		if uf.Find(a) != uf.Find(b) {
			result -= 1
			uf.Union(a, b)
		}
	}

	return result - 1
}

type input struct {
	n           int
	connections [][]int
}

func main() {
	inputs := []input{
		{n: 4, connections: [][]int{{0, 1}, {0, 2}, {1, 2}}},
		{n: 6, connections: [][]int{{0, 1}, {0, 2}, {0, 3}, {1, 2}, {1, 3}}},
		{n: 6, connections: [][]int{{0, 1}, {0, 2}, {0, 3}, {1, 2}}},
	}

	for _, input := range inputs {
		result := makeConnected(input.n, input.connections)
		fmt.Println(result)
	}
}
