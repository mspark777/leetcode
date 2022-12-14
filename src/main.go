package main

import (
	"fmt"
)

func rob(nums []int) int {
	robbed := 0
	noRobbed := 0

	for _, n := range nums {
		temp := noRobbed
		if robbed > noRobbed {
			noRobbed = robbed
		}

		robbed = n + temp
	}

	if robbed > noRobbed {
		return robbed
	}

	return noRobbed
}

func main() {
	inputs := [][]int{
		{1, 2, 3, 1},
		{2, 7, 9, 3, 1},
	}

	for _, nums := range inputs {
		result := rob(nums)
		fmt.Println(result)
	}
}
