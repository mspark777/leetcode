package main

import (
	"fmt"
)

func intersection(nums1 []int, nums2 []int) []int {
	filter1 := map[int]bool{}
	filter2 := map[int]bool{}

	for _, num := range nums1 {
		filter1[num] = true
	}

	for _, num := range nums2 {
		filter2[num] = true
	}

	result := []int{}
	for num := range filter1 {
		if _, ok := filter2[num]; ok {
			result = append(result, num)
		}
	}

	return result
}

func main() {
	inputs := [][][]int{
		{{1, 2, 2, 1}, {2, 2}},
		{{4, 9, 5}, {9, 4, 9, 8, 4}},
	}

	for _, input := range inputs {
		result := intersection(input[0], input[1])
		fmt.Println(result)
	}
}
