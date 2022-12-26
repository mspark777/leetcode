package main

import (
	"fmt"
)

func canJump(nums []int) bool {
	last := len(nums) - 1
	for i := last - 1; i >= 0; i -= 1 {
		cur := i + nums[i]
		if cur >= last {
			last = i
		}
	}

	return last < 1
}

func main() {
	inputs := [][]int{
		{2, 3, 1, 1, 4},
		{3, 2, 1, 0, 4},
	}

	for _, nums := range inputs {
		result := canJump(nums)
		fmt.Println(result)
	}
}
