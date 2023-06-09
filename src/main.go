package main

import (
	"fmt"
)

func nextGreatestLetter(letters []byte, target byte) byte {
	left := 0
	right := len(letters) - 1
	for left <= right {
		middle := (left + right) / 2
		letter := letters[middle]
		if letter <= target {
			left = middle + 1
		} else {
			right = middle - 1
		}
	}

	if left < len(letters) {
		return letters[left]
	}

	return letters[0]
}

type input struct {
	letters []byte
	target  byte
}

func main() {
	inputs := []input{
		{letters: []byte{'c', 'f', 'j'}, target: 'a'},
		{letters: []byte{'c', 'f', 'j'}, target: 'c'},
		{letters: []byte{'x', 'x', 'y', 'y'}, target: 'z'},
	}

	for _, input := range inputs {
		result := nextGreatestLetter(input.letters, input.target)
		fmt.Println(result)
	}
}
