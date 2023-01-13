package main

import (
	"fmt"
)

type solution struct {
	result int
}

func (solution *solution) dfs(current int, children [][]int, s string) int {
	longestChain := 0
	secondLongestChain := 0

	for _, child := range children[current] {
		childLongestChain := solution.dfs(child, children, s)
		if s[current] == s[child] {
			continue
		}

		if childLongestChain > longestChain {
			secondLongestChain = longestChain
			longestChain = childLongestChain
		} else if childLongestChain > secondLongestChain {
			secondLongestChain = childLongestChain
		}
	}

	result := longestChain + secondLongestChain + 1
	if result > solution.result {
		solution.result = result
	}

	return longestChain + 1
}

func longestPath(parent []int, s string) int {
	children := make([][]int, len(parent))
	for i := 0; i < len(parent); i += 1 {
		children[i] = []int{}
	}

	for i := 1; i < len(parent); i += 1 {
		children[parent[i]] = append(children[parent[i]], i)
	}

	solution := solution{result: 1}
	solution.dfs(0, children, s)
	return solution.result
}

type input struct {
	parent []int
	s      string
}

func main() {
	inputs := []input{
		{parent: []int{-1, 0, 0, 1, 1, 2}, s: "abacbe"},
		{parent: []int{-1, 0, 0, 0}, s: "aabc"},
	}

	for _, input := range inputs {
		result := longestPath(input.parent, input.s)
		fmt.Println(result)
	}
}
