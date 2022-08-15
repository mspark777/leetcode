package main

import (
	"fmt"
)

func reverse(chars []rune) []rune {
	i := 0
	j := len(chars) - 1
	for i < j {
		temp := chars[i]
		chars[i] = chars[j]
		chars[j] = temp

		i += 1
		j -= 1
	}

	return chars
}

func convertToTitle(columnNumber int) string {
	n := columnNumber
	result := []rune{}
	const ACODE rune = rune('A')
	const SIZE int = 26

	for n > 0 {
		n -= 1

		temp := ACODE + rune((n % SIZE))
		result = append(result, temp)

		n /= SIZE
	}

	return string(reverse(result))
}

type input struct {
	columnNumber int
}

func main() {
	inputs := []*input{
		{
			columnNumber: 1,
		},
		{
			columnNumber: 28,
		},
		{
			columnNumber: 701,
		},
	}

	for _, input := range inputs {
		result := convertToTitle(input.columnNumber)
		fmt.Println(result)
	}
}
