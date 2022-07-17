package main

import (
	"fmt"
	"leetcode/src/solution"
)

type input struct {
	n int
	k int
}

func main() {
	inputs := []input{
		{
			n: 3,
			k: 0,
		},
		{
			n: 3,
			k: 1,
		},
		{
			n: 3,
			k: 3,
		},
	}

	for _, input := range inputs {
		result := solution.KInversePairs(input.n, input.k)
		fmt.Printf("%v\n", result)
	}
}
