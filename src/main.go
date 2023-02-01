package main

import (
	"fmt"
)

func gcd(x, y int) int {
	if y != 0 {
		return gcd(y, x%y)
	}

	return x
}

func gcdOfStrings(str1 string, str2 string) string {
	if (str1 + str2) != (str2 + str1) {
		return ""
	}

	gcdlen := gcd(len(str1), len(str2))
	return str1[0:gcdlen]
}

func main() {
	inputs := [][]string{
		{"ABCABC", "ABC"},
		{"ABABAB", "ABAB"},
		{"LEET", "CODE"},
	}

	for _, input := range inputs {
		result := gcdOfStrings(input[0], input[1])
		fmt.Println(result)
	}
}
