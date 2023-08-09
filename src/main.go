package main

import (
	"fmt"
	"sort"
)

func countValidPairs(nums []int, threshold int) int {
	index := 1
	count := 0
	for index < len(nums) {
		first := nums[index-1]
		second := nums[index]
		diff := second - first
		if diff <= threshold {
			count += 1
			index += 1
		}

		index += 1
	}

	return count
}

func minimizeMax(nums []int, p int) int {
	sort.Ints(nums)

	left := 0
	right := nums[len(nums)-1] - nums[0]
	for left < right {
		mid := (left + right) / 2
		if countValidPairs(nums, mid) >= p {
			right = mid
		} else {
			left = mid + 1
		}
	}

	return left
}

type input struct {
	nums []int
	p    int
}

func main() {
	inputs := []input{
		{nums: []int{10, 1, 2, 7, 1, 3}, p: 2},
		{nums: []int{4, 2, 1, 2}, p: 1},
	}

	for _, input := range inputs {
		result := minimizeMax(input.nums, input.p)
		fmt.Println(result)
	}
}
