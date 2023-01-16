package main

import (
	"fmt"
)

func isPerfectSquare(num int) bool {
	left := 1
	right := num / 2
	for left <= right {
		mid := (left + right) / 2
		square := mid * mid
		if num < square {
			right = mid - 1
		} else if num > square {
			left = mid + 1
		} else {
			return true
		}
	}

	return num == 1
}

func main() {
	inputs := []int{
		16, 14, 1,
	}

	for _, input := range inputs {
		result := isPerfectSquare(input)
		fmt.Println(result)
	}
}
