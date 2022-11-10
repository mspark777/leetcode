package main

import "fmt"

func removeDuplicates(s string) string {
	result := []rune{}

	for _, c := range s {
		last := len(result) - 1
		if (last >= 0) && (result[last] == c) {
			result = result[:last]
		} else {
			result = append(result, c)
		}
	}

	return string(result)
}

func main() {
	inputs := []string{
		"abbaca",
		"azxxzy",
	}

	for _, s := range inputs {
		result := removeDuplicates(s)
		fmt.Println(result)
	}
}
