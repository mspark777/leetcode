package main

import (
	"fmt"
	"strings"
)

func simplifyPath(path string) string {
	segments := strings.Split(path, "/")
	result := []string{}

	for _, seg := range segments {
		if seg == "" {
			continue
		} else if seg == "." {
			continue
		} else if seg == ".." {
			last := len(result) - 1
			if last >= 0 {
				result = result[:last]
			}
		} else {
			result = append(result, seg)
		}
	}

	return "/" + strings.Join(result, "/")
}

func main() {
	inputs := []string{
		"/home/",
		"/../",
		"/home//foo/",
	}

	for _, path := range inputs {
		result := simplifyPath(path)
		fmt.Println(result)
	}
}
