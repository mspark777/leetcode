package main

import (
	"fmt"
	"sort"
)

func lengthOfLIS(nums []int) int {
	result := []int{}

	for _, num := range nums {
		index := sort.SearchInts(result, num)

		if index == len(result) {
			result = append(result, num)
		} else if num < result[index] {
			result[index] = num
		}
	}

	return len(result)
}

type input struct {
	nums []int
}

func main() {
	inputs := []*input{
		{
			nums: []int{10, 9, 2, 5, 3, 7, 101, 18},
		},
		{
			nums: []int{0, 1, 0, 3, 2, 3},
		},
		{
			nums: []int{7, 7, 7, 7, 7, 7, 7},
		},
	}

	for _, input := range inputs {
		nums := input.nums
		result := lengthOfLIS(nums)
		fmt.Println(result)
	}
}
