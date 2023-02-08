package main

import (
	"fmt"
)

func jump(nums []int) int {
	result := 0
	curend := 0
	curfar := 0

	for i := 0; i < len(nums)-1; i += 1 {
		next := i + nums[i]
		if next > curfar {
			curfar = next
		}

		if i == curend {
			result += 1
			curend = curfar
		}
	}

	return result
}

func main() {
	inputs := [][]int{
		{2, 3, 1, 1, 4},
		{2, 3, 0, 1, 4},
	}

	for _, input := range inputs {
		result := jump(input)
		fmt.Println(result)
	}
}
