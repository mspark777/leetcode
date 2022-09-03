package main

import (
	"fmt"
)

type stackNode struct {
	len   int
	num   int
	digit int
}

func absdiff(i, j int) int {
	if i > j {
		return i - j
	}

	return j - i
}

func numsSameConsecDiff(n int, k int) []int {
	stack := []*stackNode{}
	result := []int{}

	for i := 1; i <= 9; i += 1 {
		stack = append(stack, &stackNode{len: n - 1, num: i, digit: i})
	}

	for len(stack) > 0 {
		topidx := len(stack) - 1
		top := stack[topidx]
		stack = stack[:topidx]

		if top.len == 0 {
			result = append(result, top.num)
			continue
		}

		for i := 0; i < 10; i += 1 {
			if absdiff(top.digit, i) == k {
				stack = append(stack, &stackNode{
					len:   top.len - 1,
					num:   top.num*10 + i,
					digit: i,
				})
			}
		}
	}

	return result
}

type input struct {
	n int
	k int
}

func main() {
	inputs := []input{
		{
			n: 3,
			k: 7,
		},
		{
			n: 2,
			k: 1,
		},
	}

	for _, input := range inputs {
		result := numsSameConsecDiff(input.n, input.k)
		fmt.Println(result)
	}
}
