package main

import (
	"fmt"
)

func compress(chars []byte) int {
	newlen := 0
	for i := 0; i < len(chars); i += 0 {
		groupLen := 0

		for j := i + groupLen; j < len(chars); j += 1 {
			if chars[i] == chars[j] {
				groupLen += 1
			} else {
				break
			}
		}

		chars[newlen] = chars[i]
		newlen += 1

		if groupLen > 1 {
			for _, ch := range []byte(fmt.Sprint(groupLen)) {
				chars[newlen] = ch
				newlen += 1
			}
		}

		i += groupLen
	}

	return newlen
}

func main() {
	inputs := [][]byte{
		[]byte{'a', 'a', 'b', 'b', 'c', 'c', 'c'},
		[]byte{'a'},
		[]byte{'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'},
	}

	for _, chars := range inputs {
		result := compress(chars)
		fmt.Println(result)
	}
}
