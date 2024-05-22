package main

import (
	"fmt"
)

func dfs(result *[][]string, s []rune, start int, current []string, dp [][]bool) {
	if start >= len(s) {
		*result = append(*result, append([]string(nil), current...))
	}

	for end := start; end < len(s); end += 1 {
		check := (s[start] == s[end]) && (((end - start) <= 2) || dp[start+1][end-1])
		if check {
			dp[start][end] = true
			current = append(current, string(s[start:end+1]))
			dfs(result, s, end+1, current, dp)
			current = current[0 : len(current)-1]
		}
	}
}

func partition(s string) [][]string {
	l := len(s)
	result := [][]string{}
	current := []string{}
	dp := make([][]bool, l)
	for i := 0; i < l; i += 1 {
		dp[i] = make([]bool, l)
	}

	dfs(&result, []rune(s), 0, current, dp)

	return result
}

type input struct {
	s string
}

func main() {
	inputs := []input{
		{s: "aab"},
		{s: "a"},
	}

	for _, input := range inputs {
		result := partition(input.s)
		fmt.Println(result)
	}
}
