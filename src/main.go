package main

import (
	"fmt"
)

func subarraysDivByK(nums []int, k int) int {
	prefix := 0
	result := 0

	modGroups := make([]int, k)
	modGroups[0] = 1

	for _, num := range nums {
		prefix = (prefix + k + (num % k)) % k
		result += modGroups[prefix]
		modGroups[prefix] += 1
	}

	return result
}

type input struct {
	nums []int
	k    int
}

func main() {
	inputs := []input{
		{nums: []int{4, 5, 0, -2, -3, 1}, k: 5},
		{nums: []int{5}, k: 9},
	}

	for _, input := range inputs {
		result := subarraysDivByK(input.nums, input.k)
		fmt.Println(result)
	}
}
