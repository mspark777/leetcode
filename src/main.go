package main

import (
	"fmt"
)

func sumEvenAfterQueries(nums []int, queries [][]int) []int {
	sum := 0
	for _, num := range nums {
		if (num % 2) == 0 {
			sum += num
		}
	}

	result := make([]int, len(queries))
	for i, query := range queries {
		index := query[1]
		num := nums[index]
		if (num % 2) == 0 {
			sum -= num
		}

		val := query[0]
		num += val
		if (num % 2) == 0 {
			sum += num
		}

		nums[index] = num
		result[i] = sum
	}

	return result
}

type input struct {
	nums    []int
	queries [][]int
}

func main() {
	inputs := []input{
		{
			nums:    []int{1, 2, 3, 4},
			queries: [][]int{{1, 0}, {-3, 1}, {-4, 0}, {2, 3}},
		},
		{
			nums:    []int{1},
			queries: [][]int{{4, 0}},
		},
	}

	for _, input := range inputs {
		nums := input.nums
		queries := input.queries
		result := sumEvenAfterQueries(nums, queries)
		fmt.Println(result)
	}
}
