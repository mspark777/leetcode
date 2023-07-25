package main

import (
	"fmt"
)

func peakIndexInMountainArray(arr []int) int {
	left := 0
	right := len(arr) - 1
	for left < right {
		mid := (left + right) / 2
		if arr[mid] < arr[mid+1] {
			left = mid + 1
		} else {
			right = mid
		}
	}

	return left
}

func main() {
	inputs := [][]int{
		{0, 1, 0},
		{0, 2, 1, 0},
		{0, 10, 5, 2},
	}

	for _, arr := range inputs {
		result := peakIndexInMountainArray(arr)
		fmt.Println(result)
	}
}
