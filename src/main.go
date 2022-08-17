package main

import (
	"fmt"
)

func titleToNumber(columnTitle string) int {
	factor := rune('A') - 1
	result := 0

	for _, code := range columnTitle {
		diff := int(code - factor)
		result = result*26 + diff
	}

	return result
}

type input struct {
	columnTitle string
}

func main() {
	inputs := []*input{
		{
			columnTitle: "A",
		},
		{
			columnTitle: "AB",
		},
		{
			columnTitle: "ZY",
		},
	}

	for _, input := range inputs {
		columnTitle := input.columnTitle
		result := titleToNumber(columnTitle)
		fmt.Println(result)
	}
}
