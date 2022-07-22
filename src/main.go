package main

import (
	"fmt"
)

type input struct {
	s string
}

func main() {
	inputs := []input{
		{
			s: "A man, a plan, a canal: Panama",
		},
		{
			s: "race a car",
		},
		{
			s: " ",
		},
	}

	for _, input := range inputs {
		result := isPalindrome(input.s)
		fmt.Printf("%v\n", result)
	}
}
