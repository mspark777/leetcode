package main

import (
	"fmt"
)

func addToArrayForm(num []int, k int) []int {
	result := []int{}
	cur := k

	for i := len(num) - 1; i >= 0; i -= 1 {
		cur += num[i]
		result = append(result, cur%10)
		cur /= 10
	}

	for cur > 0 {
		result = append(result, cur%10)
		cur /= 10
	}

	for i, j := 0, len(result)-1; i < j; i, j = i+1, j-1 {
		result[i], result[j] = result[j], result[i]
	}

	return result
}

type input struct {
	num []int
	k   int
}

func main() {
	inputs := []input{
		{num: []int{1, 2, 0, 0}, k: 34},
		{num: []int{2, 7, 4}, k: 181},
		{num: []int{2, 1, 5}, k: 806},
		{num: []int{1, 2, 6, 3, 0, 7, 1, 7, 1, 9, 7, 5, 6, 6, 4, 4, 0, 0, 6, 3}, k: 516},
		{num: []int{0}, k: 10000},
	}

	for _, input := range inputs {
		result := addToArrayForm(input.num, input.k)
		fmt.Println(result)
	}
}
