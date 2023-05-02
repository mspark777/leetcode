package main

import (
	"fmt"
)

func arraySign(nums []int) int {
	result := 1
	for _, num := range nums {
		if num == 0 {
			return 0
		}

		if num < 0 {
			result *= -1
		}
	}

	if result < 0 {
		return -1
	}

	return 1
}

func main() {
	inputs := [][]int{
		{-1, -2, -3, -4, 3, 2, 1},
		{1, 5, 0, 2, -3},
		{-1, 1, -1, 1, -1},
		{9, 72, 34, 29, -49, -22, -77, -17, -66, -75, -44, -30, -24},
	}

	for _, nums := range inputs {
		result := arraySign(nums)
		fmt.Println(result)
	}
}
