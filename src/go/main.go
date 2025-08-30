package main

import (
	"fmt"
)

func maximumTime(time string) string {
	chars := []rune(time)

	if chars[0] == '?' {
		if chars[1] == '?' {
			chars[0] = '2'
		} else if chars[1] > '3' {
			chars[0] = '1'
		} else {
			chars[0] = '2'
		}
	}

	if chars[1] == '?' {
		if chars[0] > '1' {
			chars[1] = '3'
		} else {
			chars[1] = '9'
		}
	}

	if chars[3] == '?' {
		chars[3] = '5'
	}

	if chars[4] == '?' {
		chars[4] = '9'
	}

	return string(chars)
}

type input struct {
	time string
}

func main() {
	inputs := []input{
		{
			time: "2?:?0",
		},
		{
			time: "0?:3?",
		},
		{
			time: "1?:22",
		},
	}

	for _, input := range inputs {
		result := maximumTime(input.time)
		fmt.Println(result)
	}
}
