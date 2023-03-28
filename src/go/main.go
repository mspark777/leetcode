package main

import (
	"fmt"
)

func dp(days, costs, memos, durations []int, i int) int {
	if i >= len(days) {
		return 0
	} else if memos[i] != 0 {
		return memos[i]
	}

	result := int(^uint(0) >> 1)
	j := i

	for d, duration := range durations {
		for j < len(days) {
			k := days[i] + duration
			if days[j] < k {
				j += 1
			} else {
				break
			}
		}

		recv := dp(days, costs, memos, durations, j) + costs[d]
		if recv < result {
			result = recv
		}
	}

	memos[i] = result
	return result
}

func mincostTickets(days []int, costs []int) int {
	memos := make([]int, len(days))
	durations := []int{1, 7, 30}

	return dp(days, costs, memos, durations, 0)
}

func main() {
	inputs := [][][]int{
		{{1, 4, 6, 7, 8, 20}, {2, 7, 15}},
		{{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31}, {2, 7, 15}},
	}

	for _, input := range inputs {
		result := mincostTickets(input[0], input[1])
		fmt.Println(result)
	}
}
