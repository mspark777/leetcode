package main

import (
	"fmt"
)

func longestSubsequence(arr []int, difference int) int {
	dp := map[int]int{}
	result := 1

	for _, num := range arr {
		before, _ := dp[num-difference]
		now := before + 1
		dp[num] = now

		if now > result {
			result = now
		}
	}

	return result
}

type input struct {
	arr        []int
	difference int
}

func main() {
	inputs := []input{
		{arr: []int{1, 2, 3, 4}, difference: 1},
		{arr: []int{1, 3, 5, 7}, difference: 1},
		{arr: []int{1, 5, 7, 8, 5, 3, 4, 2, 1}, difference: -2},
	}

	for _, input := range inputs {
		result := longestSubsequence(input.arr, input.difference)
		fmt.Println(result)
	}
}
