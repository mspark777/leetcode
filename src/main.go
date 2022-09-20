package main

import (
	"fmt"
)

func findLength(nums1 []int, nums2 []int) int {
	result := 0

	lengths := make([][]int, len(nums1)+1)
	for i := 0; i <= len(nums1); i += 1 {
		lengths[i] = make([]int, len(nums2)+1)
	}

	for i := len(nums1) - 1; i >= 0; i -= 1 {
		for j := len(nums2) - 1; j >= 0; j -= 1 {
			if nums1[i] != nums2[j] {
				continue
			}

			length := lengths[i+1][j+1] + 1
			lengths[i][j] = length
			if length > result {
				result = length
			}
		}
	}

	return result
}

type input struct {
	nums1 []int
	nums2 []int
}

func main() {
	inputs := []input{
		{
			nums1: []int{1, 2, 3, 2, 1},
			nums2: []int{3, 2, 1, 4, 7},
		},
		{
			nums1: []int{0, 0, 0, 0, 0},
			nums2: []int{0, 0, 0, 0, 0},
		},
	}

	for _, input := range inputs {
		nums1 := input.nums1
		nums2 := input.nums2
		result := findLength(nums1, nums2)
		fmt.Println(result)
	}
}
