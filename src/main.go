package main

import (
	"fmt"
)

func makeGood(s string) string {
	chars := []rune(s)
	j := 0

	for i, cur := range chars {
		if j > 0 {
			next := chars[j-1]
			diff := cur - next
			if (diff == 32) || (diff == -32) {
				j -= 1
				continue
			}
		}

		chars[j] = chars[i]
		j += 1
	}

	return string(chars[0:j])
}

func main() {
	inputs := []string{
		"leEeetcode",
		"abBAcC",
		"s",
	}

	for _, s := range inputs {
		result := makeGood(s)
		fmt.Println(result)
	}
}
