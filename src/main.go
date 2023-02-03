package main

import (
	"fmt"
)

func convert(s string, numRows int) string {
	if numRows <= 1 {
		return s
	}

	lastRow := numRows - 1
	row := 0
	down := true

	result := make([][]rune, numRows)
	for i := 0; i < numRows; i += 1 {
		result[i] = []rune{}
	}

	for _, ch := range s {
		result[row] = append(result[row], ch)
		if row == lastRow {
			down = false
		} else if row == 0 {
			down = true
		}

		if down {
			row += 1
		} else {
			row -= 1
		}
	}

	converted := []rune{}
	for _, chars := range result {
		converted = append(converted, chars...)
	}

	return string(converted)
}

type input struct {
	s       string
	numRows int
}

func main() {
	inputs := []input{
		{s: "PAYPALISHIRING", numRows: 3},
		{s: "PAYPALISHIRING", numRows: 4},
		{s: "A", numRows: 1},
		{s: "AB", numRows: 1},
	}

	for _, input := range inputs {
		result := convert(input.s, input.numRows)
		fmt.Println(result)
	}
}
