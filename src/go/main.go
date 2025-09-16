package main

import (
	"fmt"
)

func minimumMoves(s string) int {
	result := 0
	i := 0

	ch := []rune(s)

	for i < len(ch) {
		if ch[i] == 'X' {
			result += 1
			i += 3
		} else {
			i += 1
		}
	}

	return result
}

type input struct {
	s string
}

func main() {
	inputs := []input{
		{
			s: "XXX",
		},
		{
			s: "XXOX",
		},
		{
			s: "OOOO",
		},
	}

	for _, input := range inputs {
		result := minimumMoves(input.s)
		fmt.Println(result)
	}
}
