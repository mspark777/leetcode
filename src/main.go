package main

import (
	"fmt"
)

func canVisitAllRooms(rooms [][]int) bool {
	seen := make([]bool, len(rooms))
	seen[0] = true

	stack := []int{0}

	for len(stack) > 0 {
		topidx := len(stack) - 1
		top := stack[topidx]
		stack = stack[0:topidx]
		for _, key := range rooms[top] {
			if !seen[key] {
				seen[key] = true
				stack = append(stack, key)
			}
		}
	}

	for _, s := range seen {
		if !s {
			return false
		}
	}

	return true
}

func main() {
	inputs := [][][]int{
		{{1}, {2}, {3}, {}},
		{{1, 3}, {3, 0, 1}, {2}, {0}},
	}

	for _, rooms := range inputs {
		result := canVisitAllRooms(rooms)
		fmt.Println(result)
	}
}
