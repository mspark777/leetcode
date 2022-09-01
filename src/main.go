package main

import (
	"fmt"
)

func addDigits(num int) int {
	if num != 0 {
		return 1 + ((num - 1) % 9)
	} else {
		return 0
	}
}

type input struct {
	num int
}

func main() {
	inputs := []input{
		{
			num: 38,
		},
		{
			num: 0,
		},
	}

	for _, input := range inputs {
		num := input.num
		result := addDigits(num)
		fmt.Println(result)
	}
}
