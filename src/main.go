package main

import (
	"fmt"
	"sort"
)

func threeSumClosest(nums []int, target int) int {
	sort.Ints(nums)

	result := 0
	diffresult := 2147483647

	for i, ni := range nums {
		j := i + 1
		k := len(nums) - 1

		for j < k {
			nj := nums[j]
			nk := nums[k]
			sum := ni + nj + nk
			diffsum := target - sum
			if diffsum < 0 {
				diffsum = -diffsum
			}

			if diffsum < diffresult {
				result = sum
				diffresult = diffsum
			}

			if sum < target {
				j += 1
			} else if sum > target {
				k -= 1
			} else {
				return sum
			}
		}
	}

	return result
}

type input struct {
	nums   []int
	target int
}

func main() {
	inputs := []input{
		{
			nums:   []int{-1, 2, 1, -4},
			target: 1,
		},
		{
			nums:   []int{0, 0, 0},
			target: 1,
		},
	}

	for _, input := range inputs {
		nums := input.nums
		target := input.target
		result := threeSumClosest(nums, target)
		fmt.Println(result)
	}
}
