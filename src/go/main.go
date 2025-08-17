package main

import (
	"fmt"
)

func xorB(n int, start int) int {
	if (n % 2) == 0 {
		return (n / 2) & 1
	} else {
		return ((n / 2) & 1) ^ (start + n - 1)
	}
}

func xorA(n int, start int) int {
	if (start & 1) == 1 {
		return (start - 1) ^ xorB(n+1, start-1)
	} else {
		return xorB(n, start)
	}
}

func xorOperation(n int, start int) int {
	result := 2 * xorA(n, start/2)
	if (n & start & 1) == 1 {
		result += 1
	}

	return result
}

type input struct {
	n     int
	start int
}

func main() {
	inputs := []input{
		{
			n:     5,
			start: 0,
		},
		{
			n:     4,
			start: 3,
		},
	}

	for _, input := range inputs {
		result := xorOperation(input.n, input.start)
		fmt.Println(result)
	}
}
