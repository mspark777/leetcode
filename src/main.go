package main

import (
	"fmt"
	"sort"
)

func findKthLargest(nums []int, k int) int {
	sort.Ints(nums)
	return nums[len(nums)-k]
}

type input struct {
	nums []int
	k    int
}

func main() {
	inputs := []input{
		{nums: []int{3, 2, 1, 5, 6, 4}, k: 2},
		{nums: []int{3, 2, 3, 1, 2, 4, 5, 5, 6}, k: 4},
	}

	for _, input := range inputs {
		result := findKthLargest(input.nums, input.k)
		fmt.Println(result)
	}
}
