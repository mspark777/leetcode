package main

import (
	"fmt"
	"math/bits"
)

func maxLength(arr []string) int {
	dp := []int{0}
	result := 0

	for _, str := range arr {
		n := 0
		dup := 0
		for _, ch := range str {
			code := ch - 'a'
			shift := 1 << code
			dup |= n & shift
			n |= shift
		}

		if dup > 0 {
			continue
		}

		for i := len(dp) - 1; i >= 0; i -= 1 {
			memo := dp[i]
			if (memo & n) != 0 {
				continue
			}

			m := memo | n
			dp = append(dp, m)

			bitCount := bits.OnesCount(uint(m))
			if bitCount > result {
				result = bitCount
			}
		}
	}

	return result
}

func main() {
	inputs := [][]string{
		{"un", "iq", "ue"},
		{"cha", "r", "act", "ers"},
		{"abcdefghijklmnopqrstuvwxyz"},
	}

	for _, arr := range inputs {
		result := maxLength(arr)
		fmt.Println(result)
	}
}
