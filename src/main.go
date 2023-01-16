package main

import (
	"fmt"
)

func findTheDifference(s string, t string) byte {
	sum := rune(0)

	for _, c := range s {
		sum ^= c
	}

	for _, c := range t {
		sum ^= c
	}

	return byte(sum)
}

func main() {
	inputs := [][]string{
		{"abcd", "abcde"},
		{"", "y"},
	}

	for _, input := range inputs {
		result := findTheDifference(input[0], input[1])
		fmt.Println(string(result))
	}
}
