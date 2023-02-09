package main

import (
	"fmt"
)

func distinctNames(ideas []string) int64 {
	groupMap := map[byte]map[string]bool{}

	for _, idea := range ideas {
		first := idea[0]
		remains := idea[1:]
		if group, ok := groupMap[first]; ok {
			group[remains] = true
		} else {
			groupMap[first] = map[string]bool{
				remains: true,
			}
		}
	}

	result := int64(0)
	groups := make([]map[string]bool, 0, len(groupMap))
	for _, group := range groupMap {
		groups = append(groups, group)
	}

	for i := 0; i < len(groups)-1; i += 1 {
		cur := groups[i]
		for j := i + 1; j < len(groups); j += 1 {
			group := groups[j]
			num := int64(0)

			for idea := range cur {
				if _, ok := group[idea]; ok {
					num += 1
				}
			}

			result += 2 * (int64(len(cur)) - num) * (int64(len(group)) - num)
		}
	}

	return result
}

func main() {
	inputs := [][]string{
		{"coffee", "donuts", "time", "toffee"},
		{"lack", "back"},
	}

	for _, input := range inputs {
		result := distinctNames(input)
		fmt.Println(result)
	}
}
