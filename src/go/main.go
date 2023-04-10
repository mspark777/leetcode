package main

import (
	"fmt"
)

func isValid(s string) bool {
	stack := []rune{}
	for _, ch := range s {
		if ch == '(' {
			stack = append(stack, ')')
		} else if ch == '{' {
			stack = append(stack, '}')
		} else if ch == '[' {
			stack = append(stack, ']')
		} else {
			last := len(stack) - 1
			if last < 0 {
				return false
			} else if stack[last] != ch {
				return false
			}

			stack = stack[:last]
		}

	}

	return len(stack) == 0
}

func main() {
	inputs := []string{
		"()", "()[]{}", "(]",
	}

	for _, s := range inputs {
		result := isValid(s)
		fmt.Println(result)
	}
}
