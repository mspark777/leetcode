package main

import (
	"fmt"
	"sort"
)

func groupAnagrams(strs []string) [][]string {
	groups := map[string][]string{}

	for _, str := range strs {
		runes := []rune(str)
		sort.Slice(runes, func(i, j int) bool {
			return runes[i] < runes[j]
		})

		key := string(runes)
		if group, ok := groups[key]; ok {
			group = append(group, str)
			groups[key] = group
		} else {
			groups[key] = []string{str}
		}
	}

	result := make([][]string, 0, len(groups))
	for _, v := range groups {
		result = append(result, v)
	}

	return result
}

func main() {
	inputs := [][]string{
		{"eat", "tea", "tan", "ate", "nat", "bat"},
		{""},
		{"a"},
	}

	for _, strs := range inputs {
		result := groupAnagrams(strs)
		fmt.Println(result)
	}
}
