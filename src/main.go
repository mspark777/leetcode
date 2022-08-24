package main

import "fmt"

func isPowerOfThree(n int) bool {
	if n <= 0 {
		return false
	}

	for (n % 3) == 0 {
		n /= 3
	}

	return n == 1
}

type input struct {
	n int
}

func main() {
	inputs := []input{
		{
			n: 27,
		},
		{
			n: 0,
		},
		{
			n: 9,
		},
		{
			n: 45,
		},
	}

	for _, input := range inputs {
		n := input.n
		result := isPowerOfThree(n)
		fmt.Println(result)
	}
}
