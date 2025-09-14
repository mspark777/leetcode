package main

import (
	"fmt"
)

func gcd(a, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}

func findGCD(nums []int) int {
	minN := nums[0]
	maxN := nums[0]

	for _, num := range nums {
		if num < minN {
			minN = num
		}
		if num > maxN {
			maxN = num
		}
	}

	return gcd(minN, maxN)
}

type input struct {
	nums []int
}

func main() {
	inputs := []input{
		{
			nums: []int{2, 5, 6, 9, 10},
		},
		{
			nums: []int{7, 5, 6, 8, 3},
		},
		{
			nums: []int{3, 3},
		},
	}

	for _, input := range inputs {
		result := findGCD(input.nums)
		fmt.Println(result)
	}
}
