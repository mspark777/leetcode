package main

import (
	"fmt"
	"sort"
)

func binarySearchLeft(arr []int, i int) int {
	left := 0
	right := len(arr)
	for left < right {
		mid := (left + right) / 2
		if arr[mid] < i {
			left = mid + 1
		} else {
			right = mid
		}
	}

	return left
}

func binarySearchRight(arr []int, i int) int {
	left := 0
	right := len(arr)
	for left < right {
		mid := (left + right) / 2
		if arr[mid] <= i {
			left = mid + 1
		} else {
			right = mid
		}
	}

	return left
}

func findTheDistanceValue(arr1 []int, arr2 []int, d int) int {
	sort.Ints(arr2)
	result := 0

	for _, num := range arr1 {
		left := binarySearchLeft(arr2, num-d)
		right := binarySearchRight(arr2, num+d)
		if left == right {
			result += 1
		}
	}

	return result
}

type input struct {
	arr1 []int
	arr2 []int
	d    int
}

func main() {
	inputs := []input{
		{
			arr1: []int{4, 5, 8},
			arr2: []int{10, 9, 1, 8},
			d:    2,
		},
		{
			arr1: []int{1, 4, 2, 3},
			arr2: []int{-4, -3, 6, 10, 20, 30},
			d:    3,
		},
		{
			arr1: []int{2, 1, 100, 3},
			arr2: []int{-5, -2, 10, -3, 7},
			d:    6,
		},
	}

	for _, input := range inputs {
		result := findTheDistanceValue(input.arr1, input.arr2, input.d)
		fmt.Println(result)
	}
}
