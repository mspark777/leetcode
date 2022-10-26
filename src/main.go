package main

import (
	"fmt"
)

func checkSubarraySum(nums []int, k int) bool {
	hash := map[int]int{0: 0}
	sum := 0

	for i, num := range nums {
		sum += num
		mod := sum % k
		if memo, ok := hash[mod]; ok {
			if memo < i {
				return true
			}
		} else {
			hash[mod] = i + 1
		}
	}

	return false
}

type input struct {
	nums []int
	k    int
}

func main() {
	inputs := []input{
		{
			nums: []int{23, 2, 4, 6, 7},
			k:    6,
		},
		{
			nums: []int{23, 2, 6, 4, 7},
			k:    6,
		},
		{

			nums: []int{23, 2, 6, 4, 7},
			k:    13,
		},
	}

	for _, input := range inputs {
		nums := input.nums
		k := input.k
		result := checkSubarraySum(nums, k)
		fmt.Println(result)
	}
}
