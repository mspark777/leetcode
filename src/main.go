package main

import (
	"fmt"
)

func recursive(n, t0, t1, t2 int) int {
	if n <= 2 {
		return t2
	}

	return recursive(n-1, t1, t2, t2+t1+t0)
}

func tribonacci(n int) int {
	if n < 1 {
		return 0
	} else if n < 3 {
		return 1
	}

	return recursive(n, 0, 1, 1)
}

func main() {
	inputs := []int{
		4, 25,
	}

	for _, input := range inputs {
		result := tribonacci(input)
		fmt.Println(result)
	}
}
