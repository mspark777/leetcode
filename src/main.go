package main

import (
	"fmt"
)

func solve(n int, memo []int) int {
	if n <= 1 {
		return n
	} else if memo[n] != 0 {
		return memo[n]
	}

	memo[n] = solve(n/2, memo)
	if (n & 1) == 1 {
		memo[n] += 1
	}

	return memo[n]
}

func countBits(n int) []int {
	result := make([]int, n+1)

	for i := 1; i <= n; i += 1 {
		result[i] = solve(i, result)
	}

	return result
}

func main() {
	inputs := []int{
		2, 5,
	}

	for _, n := range inputs {
		result := countBits(n)
		fmt.Println(result)
	}
}
