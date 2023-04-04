package main

import (
	"fmt"
)

func partitionString(s string) int {
	seens := make([]int, 26)
	for i := 0; i < 26; i += 1 {
		seens[i] = -1
	}

	count := 1
	substart := 0
	const ACODE = rune('a')

	for i, ch := range s {
		code := ch - ACODE
		if seens[code] >= substart {
			count += 1
			substart = i
		}
		seens[code] = i
	}

	return count
}

func main() {
	inputs := []string{
		"abacaba",
		"ssssss",
	}

	for _, input := range inputs {
		result := partitionString(input)
		fmt.Println(result)
	}
}
