package main

import (
	"fmt"
)

func dfs(node, prev int, adjMat [][]int, hasApple []bool) int {
	totalTime := 0
	childTime := 0

	for _, child := range adjMat[node] {
		if child == prev {
			continue
		}

		childTime = dfs(child, node, adjMat, hasApple)
		if (childTime > 0) || hasApple[child] {
			totalTime += childTime + 2
		}
	}

	return totalTime
}

func minTime(n int, edges [][]int, hasApple []bool) int {
	adjMat := make([][]int, n)
	for i := 0; i < n; i += 1 {
		adjMat[i] = []int{}
	}

	for _, edge := range edges {
		l := edge[0]
		r := edge[1]

		adjMat[l] = append(adjMat[l], r)
		adjMat[r] = append(adjMat[r], l)
	}

	return dfs(0, -1, adjMat, hasApple)
}

type input struct {
	n        int
	edges    [][]int
	hasApple []bool
}

func main() {
	inputs := []input{
		{n: 7, edges: [][]int{{0, 1}, {0, 2}, {1, 4}, {1, 5}, {2, 3}, {2, 6}}, hasApple: []bool{false, false, true, false, true, true, false}},
		{n: 7, edges: [][]int{{0, 1}, {0, 2}, {1, 4}, {1, 5}, {2, 3}, {2, 6}}, hasApple: []bool{false, false, true, false, false, true, false}},
		{n: 7, edges: [][]int{{0, 1}, {0, 2}, {1, 4}, {1, 5}, {2, 3}, {2, 6}}, hasApple: []bool{false, false, false, false, false, false, false}},
	}

	for _, input := range inputs {
		result := minTime(input.n, input.edges, input.hasApple)
		fmt.Println(result)
	}
}
