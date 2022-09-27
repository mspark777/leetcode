package main

import (
	"fmt"
)

func pushDominoes(dominoes string) string {
	const LEFT = 'L'
	const RIGHT = 'R'
	const STAND = '.'
	length := len(dominoes)
	forces := make([]int, length)

	chars := []rune(dominoes)

	force := 0
	for i, ch := range chars {
		if ch == LEFT {
			force = 0
		} else if ch == RIGHT {
			force = length
		} else {
			if force > 0 {
				force -= 1
			} else {
				force = 0
			}
		}

		forces[i] += force
	}

	force = 0
	for i := range chars {
		idx := length - i - 1
		ch := chars[idx]
		if ch == LEFT {
			force = length
		} else if ch == RIGHT {
			force = 0
		} else {
			if force > 0 {
				force -= 1
			} else {
				force = 0
			}
		}

		forces[idx] -= force
	}

	result := make([]rune, length)
	for i, force := range forces {
		if force < 0 {
			result[i] = LEFT
		} else if force > 0 {
			result[i] = RIGHT
		} else {
			result[i] = STAND
		}
	}

	return string(result)
}

func main() {
	inputs := []string{
		"RR.L", ".L.R...LR..L..",
	}

	for _, input := range inputs {
		result := pushDominoes(input)
		fmt.Println(result)
	}
}
