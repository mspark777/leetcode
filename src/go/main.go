package main

import (
	"fmt"
)

func filter(nums1, nums2 []int) []int {
	set1 := make(map[int]bool, len(nums2))
	for _, num := range nums2 {
		set1[num] = true
	}

	set2 := map[int]bool{}
	for _, num := range nums1 {
		if _, ok := set1[num]; !ok {
			set2[num] = true
		}
	}

	result := make([]int, 0, len(set2))
	for num := range set2 {
		result = append(result, num)
	}

	return result
}

func findDifference(nums1, nums2 []int) [][]int {
	return [][]int{
		filter(nums1, nums2),
		filter(nums2, nums1),
	}
}

func main() {
	inputs := [][][]int{
		{{1, 2, 3}, {2, 4, 6}},
		{{1, 2, 3, 3}, {1, 1, 2, 2}},
	}

	for _, input := range inputs {
		result := findDifference(input[0], input[1])
		fmt.Println(result)
	}
}
