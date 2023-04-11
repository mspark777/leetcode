package main

import (
	"fmt"
)

func removeStars(s string) string {
	stack := make([]rune, 0, len(s))
	for _, ch := range s {
		if ch == '*' {
			last := len(stack) - 1
			if last >= 0 {
				stack = stack[:last]
			}
		} else {
			stack = append(stack, ch)
		}
	}

	return string(stack)
}

func main() {
	inputs := []string{
		"leet**cod*e",
		"erase*****",
	}

	for _, s := range inputs {
		result := removeStars(s)
		fmt.Println(result)
	}
}
