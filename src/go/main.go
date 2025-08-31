package main

import (
	"fmt"
)

func canChoose(groups [][]int, nums []int) bool {
	i := 0
	n := len(nums)
	g := len(groups)
	for j := 0; (i < g) && (j < n); j += 1 {
		group := groups[i]
		l := len(group)
		match := true
		if (j + l) > n {
			return false
		}

		for k := 0; k < l; k += 1 {
			if group[k] != nums[j+k] {
				match = false
				break
			}
		}

		if match {
			j += l - 1
			i += 1
		}
	}

	return i == g
}

type input struct {
	groups [][]int
	nums   []int
}

func main() {
	inputs := []input{
		{
			groups: [][]int{{1, -1, -1}, {3, -2, 0}},
			nums:   []int{1, -1, 0, 1, -1, -1, 3, -2, 0},
		},
		{
			groups: [][]int{{10, -2}, {1, 2, 3, 4}},
			nums:   []int{1, 2, 3, 4, 10, -2},
		},
		{
			groups: [][]int{{1, 2, 3}, {3, 4}},
			nums:   []int{7, 7, 1, 2, 3, 4, 7, 7},
		},
	}

	for _, input := range inputs {
		result := canChoose(input.groups, input.nums)
		fmt.Println(result)
	}
}
