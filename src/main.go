package main

import (
	"fmt"
)

func minimumRounds(tasks []int) int {
	counts := make(map[int]int)
	for _, task := range tasks {
		counts[task] += 1
	}

	result := 0

	for _, count := range counts {
		if count == 1 {
			return -1
		}

		result += count / 3
		if (count % 3) != 0 {
			result += 1
		}
	}

	return result
}

func main() {
	inputs := [][]int{
		{2, 2, 3, 3, 2, 4, 4, 4, 4, 4},
		{2, 3, 3},
	}

	for _, tasks := range inputs {
		result := minimumRounds(tasks)
		fmt.Println(result)
	}
}
