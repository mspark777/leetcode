package main

import (
	"fmt"
)

func uniquePaths(m int, n int) int {
	total := m + n - 2
	r := n
	if r < m {
		r -= 1
	} else {
		r = m - 1
	}

	steps := 1
	for i := 1; i <= r; i += 1 {
		steps = steps * total / i
		total -= 1
	}

	return steps
}

type input struct {
	m int
	n int
}

func main() {
	inputs := []*input{
		{
			m: 3,
			n: 7,
		},
		{
			m: 3,
			n: 2,
		},
	}

	for _, input := range inputs {
		m := input.m
		n := input.n
		result := uniquePaths(m, n)
		fmt.Println(result)
	}
}
