package main

import (
	"fmt"
)

func minFlipsMonoIncr(s string) int {
	result := 0
	num := 0

	for _, c := range s {
		if c == rune('0') {
			result += 1
			if result > num {
				result = num
			}
		} else {
			num += 1
		}
	}

	return result
}

func main() {
	inputs := []string{
		"00110",
		"010110",
		"00011000",
	}

	for _, input := range inputs {
		result := minFlipsMonoIncr(input)
		fmt.Println(result)
	}
}
