package main

import (
	"fmt"
)

func increasingTriplet(nums []int) bool {
	if len(nums) < 3 {
		return false
	}

	min := 2147483647
	middle := min

	for _, n := range nums {
		if n <= min {
			min = n
		} else if n <= middle {
			middle = n
		} else {
			return true
		}
	}

	return false
}

func main() {
	inputs := [][]int{
		{1, 2, 3, 4, 5},
		{5, 4, 3, 2, 1},
		{2, 1, 5, 0, 4, 6},
		{2, 6, 1, 8},
	}

	for _, nums := range inputs {
		result := increasingTriplet(nums)
		fmt.Println(result)
	}
}
