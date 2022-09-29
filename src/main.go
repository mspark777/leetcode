package main

import (
	"fmt"
)

func findClosestElements(arr []int, k int, x int) []int {
	left := 0
	right := len(arr) - k

	for left < right {
		mid := (left + right) / 2
		a := arr[mid+k] - x
		b := x - arr[mid]
		if a < b {
			left = mid + 1
		} else {
			right = mid
		}
	}

	return arr[left : left+k]
}

type input struct {
	arr []int
	k   int
	x   int
}

func main() {
	inputs := []*input{
		{
			arr: []int{1, 2, 3, 4, 5},
			k:   4,
			x:   3,
		}, {
			arr: []int{1, 2, 3, 4, 5},
			k:   4,
			x:   -1,
		},
	}

	for _, input := range inputs {
		arr := input.arr
		k := input.k
		x := input.x
		result := findClosestElements(arr, k, x)
		fmt.Println(result)
	}
}
