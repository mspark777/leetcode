package main

import (
	"fmt"
)

func getNext(n int) int {
	result := 0
	for n > 0 {
		d := n % 10
		result += d * d
		n /= 10
	}

	return result
}

func isHappy(n int) bool {
	slow := n
	fast := getNext(n)

	for (fast != 1) && (slow != fast) {
		slow = getNext(slow)
		fast = getNext(getNext(fast))
	}

	return fast == 1
}

type input struct {
	n int
}

func main() {
	inputs := []*input{
		{
			n: 19,
		},
		{
			n: 2,
		},
	}

	for _, input := range inputs {
		n := input.n
		result := isHappy(n)
		fmt.Println(result)
	}
}
