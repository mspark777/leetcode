package main

import (
	"fmt"
	"strings"
)

func transform(s string) string {
	result := make([]string, len(s))
	indexMapping := make(map[rune]int)

	for i, ch := range s {
		if _, ok := indexMapping[ch]; !ok {
			indexMapping[ch] = i
		}

		result[i] = fmt.Sprint(indexMapping[ch])
	}

	return strings.Join(result, " ")
}

func isIsomorphic(s string, t string) bool {
	return transform(s) == transform(t)
}

type input struct {
	s string
	t string
}

func main() {
	inputs := []*input{
		{
			s: "egg",
			t: "add",
		},
		{
			s: "foo",
			t: "bar",
		},
		{
			s: "paper",
			t: "title",
		},
	}

	for _, input := range inputs {
		s := input.s
		t := input.t
		result := isIsomorphic(s, t)
		fmt.Println(result)
	}
}
