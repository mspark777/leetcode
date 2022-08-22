package main

import (
	"fmt"
)

func isPowerOfFour(n int) bool {
	return (n > 0) && ((n & (n - 1)) == 0) && ((n & 0x55555555) != 0)
}

type input struct {
	n int
}

func main() {
	inputs := []*input{
		{
			n: 16,
		},
		{
			n: 5,
		},
		{
			n: 1,
		},
	}

	for _, input := range inputs {
		n := input.n
		result := isPowerOfFour(n)
		fmt.Println(result)
	}
}
