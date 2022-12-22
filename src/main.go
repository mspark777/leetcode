package main

import (
	"fmt"
)

type dfs struct {
	result []int
	counts []int
	graph  []map[int]bool
	n      int
}

func newdfs(n int) *dfs {
	graph := []map[int]bool{}
	result := make([]int, n)
	counts := make([]int, n)

	for i := 0; i < n; i += 1 {
		counts[i] = 1
	}

	return &dfs{
		result,
		counts,
		graph,
		n,
	}
}

func (dfs *dfs) dfs(node, parent int) {
	for child := range dfs.graph[node] {
		if child != parent {
			dfs.dfs(child, node)
			dfs.counts[node] += dfs.counts[child]
			dfs.result[node] += dfs.result[child] + dfs.counts[child]
		}
	}
}

func (dfs *dfs) dfs2(node, parent int) {
	for child := range dfs.graph[node] {
		if child != parent {
			dfs.result[child] = dfs.result[node] - dfs.counts[child] + dfs.n - dfs.counts[child]
			dfs.dfs2(child, node)
		}
	}
}

func (dfs *dfs) sumOfDistancesInTree(edges [][]int) []int {
	for i := 0; i < dfs.n; i += 1 {
		dfs.graph = append(dfs.graph, map[int]bool{})
	}

	for _, edge := range edges {
		dfs.graph[edge[0]][edge[1]] = true
		dfs.graph[edge[1]][edge[0]] = true
	}

	dfs.dfs(0, -1)
	dfs.dfs2(0, -1)
	return dfs.result
}

func sumOfDistancesInTree(n int, edges [][]int) []int {
	dfs := newdfs(n)
	return dfs.sumOfDistancesInTree(edges)
}

type input struct {
	n     int
	edges [][]int
}

func main() {
	inputs := []input{
		{
			n:     6,
			edges: [][]int{{0, 1}, {0, 2}, {2, 3}, {2, 4}, {2, 5}},
		},
		{
			n:     1,
			edges: [][]int{},
		},
		{
			n:     2,
			edges: [][]int{{1, 0}},
		},
	}

	for _, input := range inputs {
		result := sumOfDistancesInTree(input.n, input.edges)
		fmt.Println(result)
	}
}
