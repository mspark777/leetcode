package main

import (
	"fmt"
	"sort"
)

func numFactoredBinaryTrees(arr []int) int {
	const MOD int64 = 1000000007
	alen := len(arr)
	sort.Ints(arr)

	dp := make([]int64, alen)
	index := make(map[int64]int64)
	for i := 0; i < alen; i += 1 {
		dp[i] = 1

		key := int64(arr[i])
		index[key] = int64(i)
	}

	for i := range arr {
		parent := int64(arr[i])
		for j := 0; j < i; j += 1 {
			left := int64(arr[j])
			if (parent % left) == 0 {
				right := parent / left
				if memo, ok := index[right]; ok {
					dp[i] += (dp[j] * dp[memo]) % MOD
				}
			}
		}
	}

	result := int64(0)
	for _, memo := range dp {
		result += memo
	}

	return int(result % MOD)
}

type input struct {
	nums []int
}

func main() {
	inputs := []*input{
		{
			nums: []int{2, 4},
		},
		{
			nums: []int{2, 4, 5, 10},
		},
	}

	for _, input := range inputs {
		nums := input.nums
		result := numFactoredBinaryTrees(nums)
		fmt.Println(result)
	}
}
