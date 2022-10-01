package main

import (
	"fmt"
)

func numDecodings(s string) int {
	bytes := []rune(s)
	slen := len(bytes)
	if slen == 0 {
		return 0
	}

	const ZERO = rune('0')

	if bytes[0] == ZERO {
		return 0
	}

	if slen == 1 {
		return 1
	}

	d1 := 1
	d2 := 1

	for i := 1; i < slen; i += 1 {
		code1 := bytes[i] - ZERO
		code0 := ((bytes[i-1] - ZERO) * 10) + code1

		n := 0

		if code1 != 0 {
			n += d1
		}

		if (10 <= code0) && (code0 <= 26) {
			n += d2
		}

		d1, d2 = n, d1
	}

	return d1
}

func main() {
	inputs := []string{
		"12", "226", "06",
	}

	for _, input := range inputs {
		result := numDecodings(input)
		fmt.Println(result)
	}
}
