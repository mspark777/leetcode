package main

import (
	"fmt"
	"strconv"
)

func maximum69Number(num int) int {
	nums := []rune(fmt.Sprint(num))
	for i, ch := range nums {
		if ch == '6' {
			nums[i] = '9'
			break
		}
	}

	i, _ := strconv.Atoi(string(nums))
	return i
}

func main() {
	inputs := []int{
		9669, 9996, 9999,
	}

	for _, num := range inputs {
		result := maximum69Number(num)
		fmt.Println(result)
	}
}
