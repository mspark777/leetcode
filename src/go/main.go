package main

import "fmt"

func makeFancyString(s string) string {
	chars := []rune(s)
	n := len(chars)
	if n < 3 {
		return s
	}

	j := 2

	for i := 2; i < n; i += 1 {
		if chars[i] != chars[j-1] || chars[i] != chars[j-2] {
			chars[j] = chars[i]
			j += 1
		}
	}

	return string(chars[0:j])
}

type input struct {
	s string
}

func main() {
	inputs := []input{
		{
			s: "leeetcode",
		},
		{
			s: "aaabaaaa",
		},
		{
			s: "aab",
		},
	}

	for _, input := range inputs {
		result := makeFancyString(input.s)
		fmt.Println(result)
	}
}
