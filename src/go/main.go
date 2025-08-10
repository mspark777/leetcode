package main

import (
	"fmt"
)

func sortString(s string) string {
	counts := make([]int, 26)
	n := 0
	for _, ch := range s {
		idx := int(ch - 'a')
		counts[idx] += 1
		n += 1
	}

	result := make([]rune, 0, n)
	for len(result) < n {
		for i := 0; i < 26; i += 1 {
			cnt := counts[i]
			if cnt > 0 {
				ch := rune(i) + 'a'
				result = append(result, ch)
				counts[i] = cnt - 1
			}
		}

		for i := 25; i >= 0; i -= 1 {
			cnt := counts[i]
			if cnt > 0 {
				ch := rune(i) + 'a'
				result = append(result, ch)
				counts[i] = cnt - 1
			}
		}
	}

	return string(result)
}

type input struct {
	s string
}

func main() {
	inputs := []input{
		{
			s: "aaaabbbbcccc",
		},
		{
			s: "rat",
		},
	}

	for _, input := range inputs {
		result := sortString(input.s)
		fmt.Println(result)
	}
}
