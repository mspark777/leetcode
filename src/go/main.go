package main

import (
	"fmt"
)

func sumOddLengthSubarrays(arr []int) int {
	n := len(arr)
	result := 0

	for i := 0; i < n; i += 1 {
		left := i
		right := n - 1 - i
		result += arr[i] * (left/2 + 1) * (right/2 + 1)
		result += arr[i] * ((left + 1) / 2) * ((right + 1) / 2)
	}

	return result
}

type input struct {
	arr []int
}

func main() {
	inputs := []input{
		{
			arr: []int{1, 4, 2, 5, 3},
		},
		{
			arr: []int{1, 2},
		},
		{
			arr: []int{10, 11, 12},
		},
	}

	for _, input := range inputs {
		result := sumOddLengthSubarrays(input.arr)
		fmt.Println(result)
	}
}
