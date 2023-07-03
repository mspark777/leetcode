package main

import (
	"fmt"
)

func buddyStrings(s string, goal string) bool {
	sbytes := []byte(s)
	gbytes := []byte(goal)
	slen := len(sbytes)
	glen := len(gbytes)
	if slen != glen {
		return false
	}

	if s == goal {
		counts := make([]int, 26)
		for _, s := range sbytes {
			i := s - byte('a')
			if counts[i] == 1 {
				return true
			} else {
				counts[i] += 1
			}
		}
	}

	first := -1
	second := -1
	for i := 0; i < slen; i += 1 {
		c := sbytes[i]
		g := gbytes[i]
		if c == g {
			continue
		}

		if first < 0 {
			first = i
		} else if second < 0 {
			second = i
		} else {
			return false
		}
	}

	if first < 0 {
		return false
	} else if second < 0 {
		return false
	}

	if sbytes[first] != gbytes[second] {
		return false
	} else if sbytes[second] != gbytes[first] {
		return false
	}

	return true
}

func main() {
	inputs := [][]string{
		{"ab", "ba"}, {"ab", "ab"}, {"aa", "aa"},
	}

	for _, input := range inputs {
		result := buddyStrings(input[0], input[1])
		fmt.Println(result)
	}
}
