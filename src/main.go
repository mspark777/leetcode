package main

import (
	"fmt"
)

func find(parents []int, code int) int {
	parent := parents[code]
	if parent == code {
		return parent
	}

	parent = find(parents, parent)
	parents[code] = parent
	return parent
}

func union(parents []int, a int, b int) {
	parenta := find(parents, a)
	parentb := find(parents, b)
	parents[parentb] = parenta
}

func equationsPossible(equations []string) bool {
	parents := make([]int, 26)
	for i := range parents {
		parents[i] = i
	}

	for _, equation := range equations {
		chars := []rune(equation)
		if chars[1] == '=' {
			a := int(chars[0] - 'a')
			b := int(chars[3] - 'a')
			union(parents, a, b)
		}
	}

	for _, equation := range equations {
		chars := []rune(equation)
		if chars[1] == '!' {
			a := int(chars[0] - 'a')
			b := int(chars[3] - 'a')
			if find(parents, a) == find(parents, b) {
				return false
			}
		}
	}

	return true
}

func main() {
	inputs := [][]string{
		{"a==b", "b!=a"},
		{"b==a", "a==b"},
		{"c==c", "b==d", "x!=z"},
	}

	for _, input := range inputs {
		result := equationsPossible(input)
		fmt.Println(result)
	}
}
