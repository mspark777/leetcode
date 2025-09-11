package main

import (
	"fmt"
)

func areOccurrencesEqual(s string) bool {
	counts := make(map[rune]int, 26)

	for _, c := range s {
		counts[c] += 1
	}

	c := 0
	for _, cnt := range counts {
		if cnt == 0 {
			continue
		} else if c == 0 {
			c = cnt
		} else if c != cnt {
			return false
		}
	}

	return true
}

type input struct {
	s string
}

func main() {
	inputs := []input{
		{

			s: "abacbc",
		},
		{
			s: "aaabb",
		},
	}

	for _, input := range inputs {
		result := areOccurrencesEqual(input.s)
		fmt.Println(result)
	}
}
