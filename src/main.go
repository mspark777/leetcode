package main

import (
	"fmt"
)

func intersect(nums1 []int, nums2 []int) []int {
	counts := map[int]int{}
	for _, num := range nums1 {
		counts[num] += 1
	}

	result := []int{}

	for _, num := range nums2 {
		count, ok := counts[num]
		if ok && (count > 0) {
			result = append(result, num)
			counts[num] = count - 1
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
		result := intersect(input[0], input[1])
		fmt.Println(result)
	}
}
