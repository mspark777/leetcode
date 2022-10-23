package main

import (
	"fmt"
)

func findErrorNums(nums []int) []int {
	temps := make([]int, len(nums))
	for _, num := range nums {
		temps[num-1] += 1
	}

	missing := -1
	dup := -1

	for i, temp := range temps {
		if temp <= 0 {
			missing = i + 1
		} else if temp > 1 {
			dup = i + 1
		}

		if (missing >= 0) && (dup >= 0) {
			break
		}
	}

	return []int{dup, missing}
}

func main() {
	inputs := [][]int{
		{1, 2, 2, 4},
		{1, 1},
	}

	for _, nums := range inputs {
		result := findErrorNums(nums)
		fmt.Println(result)
	}
}
