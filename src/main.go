package main

import (
	"fmt"
)

type unionFind struct {
	roots []int
	ranks []int
}

func newUnionFind(n int) *unionFind {
	roots := make([]int, n)
	ranks := make([]int, n)
	for i := 0; i < n; i += 1 {
		roots[i] = i
		ranks[i] = 1
	}

	return &unionFind{roots, ranks}
}

func (uf *unionFind) find(x int) int {
	roots := uf.roots
	if roots[x] != x {
		roots[x] = uf.find(roots[x])
	}

	return roots[x]
}

func (uf *unionFind) union(x, y int) {
	rootx := uf.find(x)
	rooty := uf.find(y)

	if rootx != rooty {
		ranks := uf.ranks
		if ranks[rootx] > ranks[rooty] {
			temp := rootx
			rootx = rooty
			rooty = temp
		}

		roots := uf.roots
		roots[rootx] = rooty
		ranks[rooty] += ranks[rootx]
	}
}

func validPath(n int, edges [][]int, source int, destination int) bool {
	uf := newUnionFind(n)
	for _, edge := range edges {
		uf.union(edge[0], edge[1])
	}

	return uf.find(source) == uf.find(destination)
}

type input struct {
	n           int
	edges       [][]int
	source      int
	destination int
}

func main() {
	inputs := []input{
		{
			n:           3,
			edges:       [][]int{{0, 1}, {1, 2}, {2, 0}},
			source:      0,
			destination: 2,
		},
		{
			n:           6,
			edges:       [][]int{{0, 1}, {0, 2}, {3, 5}, {5, 4}, {4, 3}},
			source:      0,
			destination: 5,
		},
	}

	for _, input := range inputs {
		result := validPath(input.n, input.edges, input.source, input.destination)
		fmt.Println(result)
	}
}
