package main

import (
	"fmt"
	"leetcode/src/solution"
)

type Input struct {
	m           int
	n           int
	maxMove     int
	startRow    int
	startColumn int
}

func main() {
	inputs := []Input{
		Input{
			m:           2,
			n:           2,
			maxMove:     2,
			startRow:    0,
			startColumn: 0,
		},
		Input{
			m:           1,
			n:           3,
			maxMove:     3,
			startRow:    0,
			startColumn: 1,
		},
		Input{
			m:           3,
			n:           2,
			maxMove:     5,
			startRow:    1,
			startColumn: 1,
		},
	}

	for _, input := range inputs {
		result := solution.FindPaths(input.m, input.n, input.maxMove, input.startRow, input.startColumn)
		fmt.Printf("%v\n", result)
	}
}
