package main

import (
	"fmt"
)

func isBipartite(graph [][]int) bool {
	const NONE int = 0
	const RED int = 1
	// const BLUE int = -1

	colors := make([]int, len(graph))
	stack := []int{}

	for i := 0; i < len(graph); i += 1 {
		if colors[i] != NONE {
			continue
		}

		colors[i] = RED
		stack = append(stack, i)
		for len(stack) > 0 {
			top := len(stack) - 1
			vertex := stack[top]
			color := colors[vertex]
			stack = stack[:top]
			for _, adjacent := range graph[vertex] {
				acolor := colors[adjacent]
				if acolor == NONE {
					colors[adjacent] = -color
					stack = append(stack, adjacent)
				} else if color == acolor {
					return false
				}
			}
		}
	}

	return true
}

func main() {
	inputs := [][][]int{
		{{1, 2, 3}, {0, 2}, {0, 1, 3}, {0, 2}},
		{{1, 3}, {0, 2}, {1, 3}, {0, 2}},
	}

	for _, graph := range inputs {
		result := isBipartite(graph)
		fmt.Println(result)
	}
}
