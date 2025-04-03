package main

import "fmt"

func max(a, b int64) int64 {
	if a > b {
		return a
	}
	return b
}

func maximumTripletValue(nums []int) int64 {
	result := int64(0)
	imax := int64(0)
	dmax := int64(0)

	for _, num := range nums {
		num := int64(num)
		result = max(result, dmax*num)
		dmax = max(dmax, imax-num)
		imax = max(imax, num)
	}

	return result
}

type input struct {
	nums []int
}

func main() {
	inputs := []input{
		{
			nums: []int{12, 6, 1, 2, 7},
		},
		{
			nums: []int{1, 10, 3, 4, 19},
		},
		{
			nums: []int{1, 2, 3},
		},
	}

	for _, input := range inputs {
		result := maximumTripletValue(input.nums)
		fmt.Println(result)
	}
}
