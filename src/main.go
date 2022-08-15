package main

import (
	"fmt"
)

func romanToInt(s string) int {
	result := 0
	num := 0

	for i := len(s) - 1; i >= 0; i -= 1 {
		ch := s[i]
		switch rune(ch) {
		case 'I':
			num = 1
		case 'V':
			num = 5
		case 'X':
			num = 10
		case 'L':
			num = 50
		case 'C':
			num = 100
		case 'D':
			num = 500
		case 'M':
			num = 1000
		}

		temp := num * 4
		if temp < result {
			result -= num
		} else {
			result += num
		}
	}

	return result
}

type input struct {
	s string
}

func main() {
	inputs := []*input{
		{
			s: "III",
		},
		{
			s: "LVIII",
		},
		{
			s: "MCMXCIV",
		},
	}

	for _, input := range inputs {
		result := romanToInt(input.s)
		fmt.Println(result)
	}
}
