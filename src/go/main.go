package main

import (
	"fmt"
)

func decode(encoded []int, first int) []int {
	result := make([]int, len(encoded)+1)

	result[0] = first
	for i := 0; i < len(encoded); i += 1 {
		result[i+1] = result[i] ^ encoded[i]
	}

	return result
}

type input struct {
	encoded []int
	first   int
}

func main() {
	inputs := []input{
		{
			encoded: []int{1, 2, 3},
			first:   1,
		},
		{
			encoded: []int{6, 2, 7, 3},
			first:   4,
		},
	}

	for _, input := range inputs {
		result := decode(input.encoded, input.first)
		fmt.Println(result)
	}
}
