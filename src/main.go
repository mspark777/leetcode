package main

import (
	"fmt"
)

func minimumDeleteSum1(s1, s2 []byte) int {
	s1len := len(s1)
	s2len := len(s2)

	cur_row := make([]int, s2len+1)
	for i := 1; i <= s2len; i += 1 {
		prev := cur_row[i-1]
		cur_row[i] = prev + int(s2[i-1])
	}

	for i := 1; i <= s1len; i += 1 {
		diag := cur_row[0]
		cur_row[0] += int(s1[i-1])

		for j := 1; j <= s2len; j += 1 {
			answer := 0
			if s1[i-1] == s2[j-1] {
				answer = diag
			} else {
				lmin := int(s1[i-1]) + cur_row[j]
				rmin := int(s2[j-1]) + cur_row[j-1]
				if lmin < rmin {
					answer = lmin
				} else {
					answer = rmin
				}
			}

			diag = cur_row[j]
			cur_row[j] = answer
		}
	}

	return cur_row[s2len]
}

func minimumDeleteSum(s1 string, s2 string) int {
	b1 := []byte(s1)
	b2 := []byte(s2)
	if len(b1) < len(b2) {
		return minimumDeleteSum1(b2, b1)
	}

	return minimumDeleteSum1(b1, b2)
}

func main() {
	inputs := [][]string{
		{"sea", "eat"},
		{"delete", "leet"},
	}

	for _, input := range inputs {
		result := minimumDeleteSum(input[0], input[1])
		fmt.Println(result)
	}
}
