package main

import (
	"fmt"
)

func validateStackSequences(pushed []int, popped []int) bool {
	numCount := len(pushed)
	stack := make([]int, 0, numCount)
	popCount := 0

	for _, p := range pushed {
		stack = append(stack, p)

		for popCount < numCount {
			top := len(stack) - 1
			if top < 0 {
				break
			} else if stack[top] != popped[popCount] {
				break
			} else {
				stack = stack[:top]
				popCount += 1
			}
		}
	}

	return popCount == numCount
}

func main() {
	inputs := [][][]int{
		{{1, 2, 3, 4, 5}, {4, 5, 3, 2, 1}},
		{{1, 2, 3, 4, 5}, {4, 3, 5, 1, 2}},
	}

	for _, input := range inputs {
		result := validateStackSequences(input[0], input[1])
		fmt.Println(result)
	}
}
