package main

import (
	"fmt"
)

func findJudge(n int, trust [][]int) int {
	counts := make([]int, n)
	for _, info := range trust {
		from := info[0] - 1
		to := info[1] - 1

		counts[from] -= 1
		counts[to] += 1
	}

	judge := n - 1
	for person, count := range counts {
		if count == judge {
			return person + 1
		}
	}

	return -1
}

type input struct {
	n     int
	trust [][]int
}

func main() {
	inputs := []input{
		{n: 2, trust: [][]int{{1, 2}}},
		{n: 3, trust: [][]int{{1, 3}, {2, 3}}},
		{n: 3, trust: [][]int{{1, 3}, {2, 3}, {3, 1}}},
		{n: 1, trust: [][]int{}},
	}

	for _, input := range inputs {
		result := findJudge(input.n, input.trust)
		fmt.Println(result)
	}
}
