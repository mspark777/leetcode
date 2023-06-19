package main

import (
	"fmt"
)

func largestAltitude(gain []int) int {
	result := 0
	current := result

	for _, altitude := range gain {
		current += altitude
		if current > result {
			result = current
		}
	}

	return result
}

func main() {
	inputs := [][]int{
		{-5, 1, 5, 0, -7},
		{-4, -3, -2, -1, 4, 3, 2},
	}

	for _, gain := range inputs {
		result := largestAltitude(gain)
		fmt.Println(result)
	}
}
