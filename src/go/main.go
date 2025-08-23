package main

import (
	"fmt"
)

func minOperations(logs []string) int {
	result := 0

	for _, operation := range logs {
		if operation == "../" {
			if result > 0 {
				result = max(result-1, 0)
			}
		} else if operation != "./" {
			result += 1
		}
	}

	return result
}

type input struct {
	logs []string
}

func main() {
	inputs := []input{
		{
			logs: []string{"d1/", "d2/", "../", "d21/", "./"},
		},
		{
			logs: []string{"d1/", "d2/", "./", "d3/", "../", "d31/"},
		},
		{
			logs: []string{"d1/", "../", "../", "../"},
		},
	}

	for _, input := range inputs {
		result := minOperations(input.logs)
		fmt.Println(result)
	}
}
