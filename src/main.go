package main

import (
	"fmt"
)

func trap(height []int) int {
	left := 0
	right := len(height) - 1
	leftMax := 0
	rightMax := 0
	result := 0

	for left < right {
		lheight := height[left]
		rheight := height[right]
		if lheight < rheight {
			left += 1
			if lheight >= leftMax {
				leftMax = lheight
			} else {
				result += leftMax - lheight
			}
		} else {
			right -= 1
			if rheight >= rightMax {
				rightMax = rheight
			} else {
				result += rightMax - rheight
			}
		}
	}

	return result
}

type input struct {
	height []int
}

func main() {
	inputs := []input{
		{
			height: []int{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1},
		},
		{
			height: []int{4, 2, 0, 3, 2, 5},
		},
	}

	for _, input := range inputs {
		height := input.height
		result := trap(height)
		fmt.Println(result)
	}
}
