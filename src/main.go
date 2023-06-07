package main

import (
	"fmt"
	"math/bits"
)

func minFlips(a int, b int, c int) int {
	ua := uint(a)
	ub := uint(b)
	uc := uint(c)
	ud := (ua | ub) ^ uc
	ue := ua & ub & ud
	return bits.OnesCount(ud) + bits.OnesCount(ue)
}

func main() {
	inputs := [][]int{
		{2, 6, 5},
		{4, 2, 7},
		{1, 2, 3},
		{7, 3, 9},
	}

	for _, input := range inputs {
		result := minFlips(input[0], input[1], input[2])
		fmt.Println(result)
	}
}
