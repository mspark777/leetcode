package main

import (
	"fmt"
)

func isPowerOfTwo(n int) bool {
	if n <= 0 {
		return false
	}

	return (n & (n - 1)) == 0
}

type input struct {
	n int
}

func main() {
	inputs := []*input{
		{
			n: 1,
		},
		{
			n: 16,
		},
		{
			n: 3,
		},
	}

	for _, input := range inputs {
		n := input.n
		result := isPowerOfTwo(n)
		fmt.Println(result)
	}
}
