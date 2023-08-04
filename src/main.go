package main

import (
	"fmt"
)

func myPow(x float64, n int) float64 {
	if n == 0 {
		return 1
	}

	if n < 0 {
		n *= -1
		x = 1 / x
	}

	result := 1.0
	for n != 0 {
		if (n & 1) == 1 {
			result *= x
			n -= 1
		}

		x *= x
		n /= 2
	}

	return result
}

type input struct {
	x float64
	n int
}

func main() {
	inputs := []input{
		{x: 2.00000, n: 10},
		{x: 2.10000, n: 3},
		{x: 2.00000, n: -2},
	}

	for _, input := range inputs {
		result := myPow(input.x, input.n)
		fmt.Println(result)
	}
}
