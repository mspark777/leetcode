package main

import (
	"fmt"
)

func getCost(nums []int, cost []int, base int) int64 {
	result := int64(0)

	for i, num := range nums {
		diff := int64(num - base)
		if diff < 0 {
			diff *= -1
		}

		result += diff * int64(cost[i])
	}

	return result
}

func minCost(nums []int, cost []int) int64 {
	left := 1000001
	right := 0
	for _, num := range nums {
		if num < left {
			left = num
		}

		if num > right {
			right = num
		}
	}

	result := getCost(nums, cost, nums[0])
	for left < right {
		mid := (left + right) / 2
		cost1 := getCost(nums, cost, mid)
		cost2 := getCost(nums, cost, mid+1)
		if cost1 < cost2 {
			result = cost1
			right = mid
		} else {
			result = cost2
			left = mid + 1
		}
	}

	return result
}

type input struct {
	nums []int
	cost []int
}

func main() {
	inputs := []input{
		{nums: []int{1, 3, 5, 2}, cost: []int{2, 3, 1, 14}},
		{nums: []int{2, 2, 2, 2, 2}, cost: []int{4, 2, 8, 1, 3}},
	}

	for _, input := range inputs {
		result := minCost(input.nums, input.cost)
		fmt.Println(result)
	}
}
