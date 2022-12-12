package main

import (
	"fmt"
)

func climbStairs(n int) int {
	n0 := 1
	n1 := 1

	for i := 1; i < n; i += 1 {
		sum := n0 + n1
		n0 = n1
		n1 = sum
	}

	return n1
}

func main() {
	inputs := []int{
		2, 3,
	}

	for _, n := range inputs {
		result := climbStairs(n)
		fmt.Println(result)
	}
}
