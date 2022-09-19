package main

import (
	"fmt"
	"strings"
)

func findDuplicate(paths []string) [][]string {
	pathMap := make(map[string][]string)
	for _, path := range paths {
		segments := strings.Split(path, " ")
		root := segments[0]
		for i := 1; i < len(segments); i += 1 {
			file := segments[i]
			sep := strings.Index(file, "(")
			name := file[0:sep]
			content := file[sep : len(file)-1]
			filepath := fmt.Sprintf("%v/%v", root, name)
			if list, ok := pathMap[content]; ok {
				pathMap[content] = append(list, filepath)
			} else {
				pathMap[content] = []string{filepath}
			}
		}
	}

	result := [][]string{}
	for _, value := range pathMap {
		if len(value) > 1 {
			result = append(result, value)
		}
	}

	return result
}

type input struct {
	paths []string
}

func main() {
	inputs := []input{
		{
			[]string{
				"root/a 1.txt(abcd) 2.txt(efgh)",
				"root/c 3.txt(abcd)",
				"root/c/d 4.txt(efgh)",
				"root 4.txt(efgh)",
			},
		},
		{
			[]string{
				"root/a 1.txt(abcd) 2.txt(efgh)",
				"root/c 3.txt(abcd)",
				"root/c/d 4.txt(efgh)",
			},
		},
		{
			[]string{
				"root/a 1.txt(abcd) 2.txt(efsfgh)",
				"root/c 3.txt(abdfcd)",
				"root/c/d 4.txt(efggdfh)",
			},
		},
	}

	for _, input := range inputs {
		paths := input.paths
		result := findDuplicate(paths)
		fmt.Println(result)
	}
}
