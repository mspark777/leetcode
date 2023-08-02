package main

import (
	"fmt"
)

func isMatch(s string, p string) bool {
	sb := []byte(s)
	pb := []byte(p)
	si := 0
	pi := 0
	mi := 0
	stari := -1

	for si < len(sb) {
		if (pi < len(pb)) && ((pb[pi] == '?') || (sb[si] == pb[pi])) {
			si += 1
			pi += 1
		} else if (pi < len(pb)) && (pb[pi] == '*') {
			stari = pi
			mi = si
			pi += 1
		} else if stari >= 0 {
			pi = stari
			mi += 1
			si = mi
		} else {
			return false
		}
	}

	for pi < len(pb) {
		if pb[pi] == '*' {
			pi += 1
		} else {
			break
		}
	}

	return pi == len(pb)
}

func main() {
	inputs := [][]string{
		{"aa", "a"},
		{"aa", "*"},
		{"cb", "?a"},
	}

	for _, input := range inputs {
		result := isMatch(input[0], input[1])
		fmt.Println(result)
	}
}
