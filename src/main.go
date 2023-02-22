package main

import (
	"fmt"
)

func feasible(weights []int, capacity, days int) bool {
	daysNeeded := 1
	currentLoad := 0

	for _, weight := range weights {
		currentLoad += weight
		if currentLoad > capacity {
			daysNeeded += 1
			currentLoad = weight
		}

		if daysNeeded > days {
			return false
		}
	}

	return true
}

func shipWithinDays(weights []int, days int) int {
	totalLoad := 0
	maxLoad := 0

	for _, weight := range weights {
		totalLoad += weight
		if weight > maxLoad {
			maxLoad = weight
		}
	}

	left := maxLoad
	right := totalLoad

	for left < right {
		middle := (left + right) / 2
		if feasible(weights, middle, days) {
			right = middle
		} else {
			left = middle + 1
		}
	}

	return left
}

type input struct {
	weights []int
	days    int
}

func main() {
	inputs := []input{
		{weights: []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}, days: 5},
		{weights: []int{3, 2, 2, 4, 1, 4}, days: 3},
		{weights: []int{1, 2, 3, 1, 1}, days: 4},
	}

	for _, input := range inputs {
		result := shipWithinDays(input.weights, input.days)
		fmt.Println(result)
	}
}
