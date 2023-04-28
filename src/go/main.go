package main

import (
	"fmt"
)

func isSimilar(a, b string) bool {
	diff := 0

	for i, ca := range a {
		cb := rune(b[i])
		if ca != cb {
			diff += 1
		}
	}

	return (diff == 0) || (diff == 2)
}

func numSimilarGroups(strs []string) int {
	strsLen := len(strs)
	uf := UnionFindConstructor(strsLen)
	result := strsLen

	for i := 0; i < strsLen; i += 1 {
		a := strs[i]
		for j := i + 1; j < strsLen; j += 1 {
			b := strs[j]
			if !isSimilar(a, b) {
				continue
			}

			if uf.Find(i) != uf.Find(j) {
				result -= 1
				uf.Union(i, j)
			}
		}
	}

	return result
}

func main() {
	inputs := [][]string{
		{"tars", "rats", "arts", "star"},
		{"omv", "ovm"},
	}

	for _, strs := range inputs {
		result := numSimilarGroups(strs)
		fmt.Println(result)
	}
}
