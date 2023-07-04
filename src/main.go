package main

import (
	"fmt"
)

func singleNumber(nums []int) int {
	a := 0
	b := 0

	for _, num := range nums {
		tempa := (a & ^b & ^num) + (^a & b & num)
		tempb := (^a & b & ^num) + (^a & ^b & num)

		a = tempa
		b = tempb
	}

	return a | b
}

func main() {
	inputs := [][]int{
		{2, 2, 3, 2},
		{0, 1, 0, 1, 0, 1, 99},
	}

	for _, input := range inputs {
		result := singleNumber(input)
		fmt.Println(result)
	}
}
