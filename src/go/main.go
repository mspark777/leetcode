package main

import (
	"fmt"
)

func countGoodSubstrings(s string) int {
	result := 0
	chars := []rune(s)
	n := len(chars)
	if n < 3 {
		return result
	}

	for i := 2; i < n; i += 1 {
		a := chars[i-2]
		b := chars[i-1]
		c := chars[i]

		if a != b && b != c && a != c {
			result += 1
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
			s: "xyzzaz",
		},
		{
			s: "aababcabc",
		},
	}

	for _, input := range inputs {
		result := countGoodSubstrings(input.s)
		fmt.Println(result)
	}
}
