package main

import (
	"fmt"
)

func sumSubarrayMins(arr []int) int {
	stack := []int{}
	dp := make([]int, len(arr))

	for i, n := range arr {
		for (len(stack) > 0) && (arr[stack[len(stack)-1]] >= n) {
			stack = stack[:len(stack)-1]
		}

		if len(stack) > 0 {
			top := stack[len(stack)-1]
			dp[i] = dp[top] + (i-top)*n
		} else {
			dp[i] = (i + 1) * n
		}

		stack = append(stack, i)
	}

	result := 0
	for _, count := range dp {
		result += count
		result %= 1000000007
	}

	return result
}

func main() {
	inputs := [][]int{
		{3, 1, 2, 4},
		{11, 81, 94, 43, 3},
	}

	for _, arr := range inputs {
		result := sumSubarrayMins(arr)
		fmt.Println(result)
	}
}
