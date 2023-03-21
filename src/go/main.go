package main

import (
	"fmt"
)

func zeroFilledSubarray(nums []int) int64 {
	result := 0
	sub := 0

	for _, num := range nums {
		if num == 0 {
			sub += 1
		} else {
			sub = 0
		}

		result += sub
	}

	return int64(result)
}

func main() {
	inputs := [][]int{
		{1, 3, 0, 0, 2, 0, 0, 4},
		{0, 0, 0, 2, 0, 0},
		{2, 10, 2019},
	}

	for _, nums := range inputs {
		result := zeroFilledSubarray(nums)
		fmt.Println(result)
	}
}
