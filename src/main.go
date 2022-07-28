package main

import (
	"fmt"
)

type input struct {
	s string
	t string
}

func main() {
	inputs := []input{
		{
			s: "anagram",
			t: "nagaram",
		},
		{
			s: "rat",
			t: "car",
		},
	}

	for _, input := range inputs {
		result := isAnagram(input.s, input.t)
		fmt.Printf("%v\n", result)
	}
}
