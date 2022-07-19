package main

import (
	"fmt"
)

type input struct {
	maxRows int
}

func main() {
	inputs := []input{
		{
			maxRows: 5,
		},
		{
			maxRows: 1,
		},
	}

	for _, input := range inputs {
		result := generate(input.maxRows)
		fmt.Printf("%v\n", result)
	}
}
