package main

import (
	"fmt"
)

func swap(nums []int, i, j int) {
	temp := nums[i]
	nums[i] = nums[j]
	nums[j] = temp
}

func reverse(nums []int, start int) {
	i := start
	j := len(nums) - 1
	for i < j {
		swap(nums, i, j)
		i += 1
		j -= 1
	}
}

func nextPermutation(nums []int) {
	i := len(nums) - 2
	for (i >= 0) && (nums[i] >= nums[i+1]) {
		i -= 1
	}

	if i >= 0 {
		j := len(nums) - 1
		for nums[i] >= nums[j] {
			j -= 1
		}

		swap(nums, i, j)
	}
	reverse(nums, i+1)
}

type input struct {
	nums []int
}

func main() {
	inputs := []*input{
		{
			nums: []int{1, 2, 3},
		},
		{
			nums: []int{3, 2, 1},
		},
		{
			nums: []int{1, 1, 5},
		},
	}

	for _, input := range inputs {
		nums := input.nums
		nextPermutation(nums)
		fmt.Println(nums)
	}
}
