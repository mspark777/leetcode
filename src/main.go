package main

import "fmt"

var guess func(n int) int = nil

func getGuess(pick int) func(n int) int {
	return func(n int) int {
		if n < pick {
			return 1
		} else if n > pick {
			return -1
		} else {
			return 0
		}
	}
}

func guessNumber(n int) int {
	left := 1
	right := n
	for left <= right {
		m := (left + right) / 2
		res := guess(m)
		if res < 0 {
			right = m - 1
		} else if res > 0 {
			left = m + 1
		} else {
			return m
		}
	}

	return -1
}

func main() {
	inputs := [][]int{
		{10, 6}, {1, 1}, {2, 1},
	}

	for _, input := range inputs {
		n := input[0]
		pick := input[1]
		guess = getGuess(pick)
		result := guessNumber(n)
		fmt.Println(result)
	}
}
