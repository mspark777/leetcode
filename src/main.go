package main

import (
	"fmt"
)

func dfs(i int, graph map[int][]int) int {
	stack := make([]int, 0, len(graph))
	visited := map[int]bool{i: true}

	stack = append(stack, i)

	for len(stack) > 0 {
		topidx := len(stack) - 1
		top := stack[topidx]
		stack = stack[:topidx]
		nodes, ok := graph[top]
		if !ok {
			continue
		}

		for _, node := range nodes {
			if _, v := visited[node]; !v {
				stack = append(stack, node)
				visited[node] = true
			}
		}
	}

	return len(visited)
}

func maximumDetonation(bombs [][]int) int {
	graph := map[int][]int{}

	for i, ibomb := range bombs {
		for j, jbomb := range bombs {
			if i == j {
				continue
			}

			dx := ibomb[0] - jbomb[0]
			dy := ibomb[1] - jbomb[1]
			d := (dx * dx) + (dy * dy)
			r := ibomb[2] * ibomb[2]
			if r < d {
				continue
			}

			nodes, ok := graph[i]
			if ok {
				graph[i] = append(nodes, j)
			} else {
				graph[i] = []int{j}
			}
		}
	}

	result := 0
	for i := range bombs {
		count := dfs(i, graph)
		if count > result {
			result = count
		}
	}

	return result
}

func main() {
	inputs := [][][]int{
		{{2, 1, 3}, {6, 1, 4}},
		{{1, 1, 5}, {10, 10, 5}},
		{{1, 2, 3}, {2, 3, 1}, {3, 4, 2}, {4, 5, 3}, {5, 6, 4}},
	}

	for _, bombs := range inputs {
		result := maximumDetonation(bombs)
		fmt.Println(result)
	}
}
