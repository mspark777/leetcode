package main

import (
	"fmt"
	"leetcode/src/solution"
)

type Input struct {
	n int
	k int
}

func main() {
	inputs := []Input{
		Input{
			n: 3,
			k: 0,
		},
		Input{
			n: 3,
			k: 1,
		},
		Input{
			n: 3,
			k: 3,
		},
	}

	for _, input := range inputs {
		result := solution.KInversePairs(input.n, input.k)
		fmt.Printf("%v\n", result)
	}
}
