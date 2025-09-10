package main

import (
	"fmt"
)

func isCovered(ranges [][]int, left int, right int) bool {
	for x := left; x <= right; x += 1 {
		ok := false
		for _, r := range ranges {
			s := r[0]
			e := r[1]

			if s <= x && x <= e {
				ok = true
				break
			}
		}

		if !ok {
			return false
		}
	}

	return true
}

type input struct {
	ranges [][]int
	left   int
	right  int
}

func main() {
	inputs := []input{
		{
			ranges: [][]int{{1, 2}, {3, 4}, {5, 6}},
			left:   2,
			right:  5,
		},
		{
			ranges: [][]int{{1, 10}, {10, 20}},
			left:   21,
			right:  21,
		},
	}

	for _, input := range inputs {
		result := isCovered(input.ranges, input.left, input.right)
		fmt.Println(result)
	}
}
