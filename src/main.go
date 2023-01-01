package main

import (
	"fmt"
)

func reverseString(s []byte) {
	i := 0
	j := len(s) - 1
	for i < j {
		s[i], s[j] = s[j], s[i]
		i += 1
		j -= 1
	}
}

func main() {
	inputs := [][]byte{
		{'h', 'e', 'l', 'l', 'o'},
		{'H', 'a', 'n', 'n', 'a', 'h'},
	}

	for _, s := range inputs {
		reverseString(s)
		fmt.Println(string(s))
	}
}
