package main

import (
	"fmt"
	"sort"
)

type frequent struct {
	num   int
	count int
}

func topKFrequent(nums []int, k int) []int {
	counts := map[int]int{}
	for _, num := range nums {
		counts[num] += 1
	}

	frequents := make([]frequent, len(counts))
	i := 0
	for num, count := range counts {
		frequents[i] = frequent{num, count}
		i += 1
	}

	sort.Slice(frequents, func(i, j int) bool {
		return frequents[j].count < frequents[i].count
	})

	frequents = frequents[0:k]
	result := make([]int, k)
	for i, frequent := range frequents {
		result[i] = frequent.num
	}

	return result
}

type input struct {
	nums []int
	k    int
}

func main() {
	inputs := []input{
		{nums: []int{1, 1, 1, 2, 2, 3}, k: 2},
		{nums: []int{1}, k: 1},
	}

	for _, input := range inputs {
		result := topKFrequent(input.nums, input.k)
		fmt.Println(result)
	}
}
