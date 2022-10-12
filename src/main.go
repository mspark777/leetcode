package main

import (
	"fmt"
	"sort"
)

func largestPerimeter(nums []int) int {
	sort.Sort(sort.Reverse(sort.IntSlice(nums)))
	for i := 0; i <= len(nums)-3; i += 1 {
		a := nums[i]
		b := nums[i+1] + nums[i+2]
		if a < b {
			return a + b
		}
	}

	return 0
}

func main() {
	inputs := [][]int{
		{2, 1, 2},
		{1, 2, 1},
	}

	for _, nums := range inputs {
		result := largestPerimeter(nums)
		fmt.Println(result)
	}
}
