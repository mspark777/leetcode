package main

import "fmt"
import "math"

func maximumUniqueSubarray(nums []int) int {
	result := math.MinInt
	sum := 0
	visits := make(map[int]int)
	left := 0
	right := 0
	for right < len(nums) {
		rn := nums[right]
		if v, ok := visits[rn]; ok {
			for left <= v {
				sum -= nums[left]
				left += 1
			}
		}

		sum += rn
		result = max(result, sum)
		visits[rn] = right
		right += 1
	}

	return result
}

type input struct {
	nums []int
}

func main() {
	inputs := []input{
		{
			nums: []int{4, 2, 4, 5, 6},
		},
		{
			nums: []int{5, 2, 1, 2, 5, 2, 1, 2, 5},
		},
	}

	for _, input := range inputs {
		result := maximumUniqueSubarray(input.nums)
		fmt.Println(result)
	}
}
