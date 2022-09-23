package main

import (
	"fmt"
)

func concatenatedBinary(n int) int {
	result := 1
	length := 4
	mod := 1000000007

	for i := 2; i <= n; i += 1 {
		if i == length {
			length *= 2
		}

		result = ((result * length) + i) % mod
	}

	return result
}

type input struct {
	n int
}

func main() {
	inputs := []input{
		{
			n: 1,
		},
		{
			n: 3,
		},
		{
			n: 12,
		},
	}

	for _, input := range inputs {
		n := input.n
		result := concatenatedBinary(n)
		fmt.Println(result)
	}
}
