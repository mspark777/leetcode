package main

import (
	"fmt"
)

func letterCombinations(digits string) []string {
	if len(digits) < 1 {
		return []string{}
	}

	lettersMap := make(map[byte]string)
	lettersMap['2'] = "abc"
	lettersMap['3'] = "def"
	lettersMap['4'] = "ghi"
	lettersMap['5'] = "jkl"
	lettersMap['6'] = "mno"
	lettersMap['7'] = "pqrs"
	lettersMap['8'] = "tuv"
	lettersMap['9'] = "wxyz"

	stack := []string{""}
	result := []string{}

	for len(stack) > 0 {
		topidx := len(stack) - 1
		top := stack[topidx]
		stack = stack[:topidx]
		ch := digits[len(top)]
		letters := lettersMap[ch]
		for _, letter := range letters {
			combination := top + string(letter)
			if len(combination) == len(digits) {
				result = append(result, combination)
			} else {
				stack = append(stack, combination)
			}
		}
	}

	return result
}

func main() {
	inputs := []string{
		"23",
		"",
		"2",
	}

	for _, input := range inputs {
		result := letterCombinations(input)
		fmt.Println(result)
	}
}
