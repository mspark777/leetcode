package main

import (
	"fmt"
)

func containsNearbyDuplicate(nums []int, k int) bool {
	indexMap := make(map[int]int)
	for i, num := range nums {
		if idx, ok := indexMap[num]; ok {
			if (i - idx) <= k {
				return true
			}
		}
		indexMap[num] = i
	}

	return false
}

type input struct {
	nums []int
	k    int
}

func main() {
	inputs := []*input{
		{
			nums: []int{1, 2, 3, 1},
			k:    3,
		},
		{
			nums: []int{1, 0, 1, 1},
			k:    1,
		},
		{
			nums: []int{1, 2, 3, 1, 2, 3},
			k:    2,
		},
	}

	for _, input := range inputs {
		nums := input.nums
		k := input.k
		result := containsNearbyDuplicate(nums, k)
		fmt.Println(result)
	}
}
