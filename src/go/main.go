package main

import (
	"fmt"
)

func findKthPositive(arr []int, k int) int {
	left := 0
	right := len(arr)
	for left < right {
		middle := (left + right) / 2
		n := arr[middle] - (middle + 1)
		if n < k {
			left = middle + 1
		} else {
			right = middle
		}
	}

	return left + k
}

type input struct {
	arr []int
	k   int
}

func main() {
	inputs := []input{
		{arr: []int{2, 3, 4, 7, 11}, k: 5},
		{arr: []int{1, 2, 3, 4}, k: 2},
	}

	for _, input := range inputs {
		result := findKthPositive(input.arr, input.k)
		fmt.Println(result)
	}
}
