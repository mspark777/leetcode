package main

import "fmt"

func reverse(arr []rune) {
	i := 0
	j := len(arr) - 1
	for i < j {
		temp := arr[i]
		arr[i] = arr[j]
		arr[j] = temp

		i += 1
		j -= 1
	}
}

func stackToString(arr []rune) string {
	reverse(arr)
	return string(arr)
}

func decodeString(s string) string {
	stack := []rune{}

	for _, ch := range s {
		if ch != ']' {
			stack = append(stack, ch)
			continue
		}

		chars := []rune{}
		for len(stack) > 0 {
			topidx := len(stack) - 1
			top := stack[topidx]
			stack = stack[:topidx]

			if top != '[' {
				chars = append(chars, top)
			} else {
				break
			}
		}

		nums := []rune{}
		for len(stack) > 0 {
			topidx := len(stack) - 1
			top := stack[topidx]
			if ('0' <= top) && (top <= '9') {
				nums = append(nums, top)
				stack = stack[:topidx]
			} else {
				break
			}
		}

		reverse(chars)
		count := 0
		fmt.Sscanf(stackToString(nums), "%d", &count)

		for i := 0; i < count; i += 1 {
			stack = append(stack, chars...)
		}
	}

	return string(stack)
}

type input struct {
	s string
}

func main() {
	inputs := []input{
		{
			s: "3[a]2[bc]",
		},
		{
			s: "3[a2[c]]",
		},
		{
			s: "2[abc]3[cd]ef",
		},
	}

	for _, input := range inputs {
		s := input.s
		result := decodeString(s)
		fmt.Println(result)
	}
}
