package main

import (
	"fmt"
	"strings"
)

func countAndSay(n int) string {
	result := []string{"1"}

	for i := 1; i < n; i += 1 {
		temp := []string{}
		count := 1
		ch := result[0]
		for j := 1; j < len(result); j += 1 {
			c := result[j]
			if ch == c {
				count += 1
			} else {
				temp = append(temp, fmt.Sprint(count), ch)

				ch = c
				count = 1
			}
		}
		temp = append(temp, fmt.Sprint(count), ch)
		result = temp
	}

	return strings.Join(result, "")
}

func main() {
	inputs := []int{1, 4}

	for _, input := range inputs {
		result := countAndSay(input)
		fmt.Println(result)
	}
}
