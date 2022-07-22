package main

import (
	"strings"
)

func isPalindrome(s string) bool {
	s = strings.ToLower(s)

	i := 0
	j := len(s) - 1
	for i < j {
		if !isAlphanumeric(s[i]) {
			i += 1
			continue
		}
		if !isAlphanumeric(s[j]) {
			j -= 1
			continue
		}

		if s[i] != s[j] {
			return false
		}
		i += 1
		j -= 1
	}

	return true
}

func isAlphanumeric(l uint8) bool {
	return ((l >= 'a' && l <= 'z') || (l >= '0' && l <= '9'))
}
