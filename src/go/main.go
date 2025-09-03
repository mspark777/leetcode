package main

import (
	"fmt"
)

func sumBase(n int, k int) int {
	result := 0

	for n > 0 {
		result += n % k
		n /= k
	}

	return result
}

type input struct {
	n int
	k int
}

func main() {
	inputs := []input{
		{
			n: 34,
			k: 6,
		},
		{
			n: 10,
			k: 10,
		},
	}

	for _, input := range inputs {
		result := sumBase(input.n, input.k)
		fmt.Println(result)
	}
}
