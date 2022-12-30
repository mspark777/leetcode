package main

import (
	"fmt"
)

type dfs struct {
	results [][]int
	path    []int
}

func (dfs *dfs) dfs(graph [][]int, cur int) {
	last := len(dfs.path)
	dfs.path = append(dfs.path, cur)

	if cur == (len(graph) - 1) {
		path := make([]int, last+1)
		copy(path, dfs.path)
		dfs.results = append(dfs.results, path)
	} else {
		for _, next := range graph[cur] {
			dfs.dfs(graph, next)
		}
	}

	dfs.path = dfs.path[0:last]
}

func allPathsSourceTarget(graph [][]int) [][]int {
	dfs := dfs{
		results: [][]int{},
		path:    []int{},
	}
	dfs.dfs(graph, 0)

	return dfs.results
}

func main() {
	inputs := [][][]int{
		{{1, 2}, {3}, {3}, {}},
		{{4, 3, 1}, {3, 2, 4}, {3}, {4}, {}},
	}

	for _, graph := range inputs {
		result := allPathsSourceTarget(graph)
		fmt.Println(result)
	}
}
