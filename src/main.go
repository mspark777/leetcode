package main

import (
	"fmt"
)

func isUgly(n int) bool {
	for n > 1 {
		if (n % 2) == 0 {
			n /= 2
		} else if (n % 3) == 0 {
			n /= 3
		} else if (n % 5) == 0 {
			n /= 5
		} else {
			return false
		}
	}

	return n == 1
}

type input struct {
	n int
}

func main() {
	inputs := []input{
		{
			n: 6,
		},
		{
			n: 1,
		},
		{
			n: 14,
		},
		{
			n: -2147483648,
		},
	}

	for _, input := range inputs {
		n := input.n
		result := isUgly(n)
		fmt.Println(result)
	}
}
