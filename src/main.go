package main

import (
	"fmt"
	"sort"
)

func maximumBags(capacity []int, rocks []int, additionalRocks int) int {
	remains := make([]int, len(capacity))
	for i, rock := range rocks {
		remains[i] = capacity[i] - rock
	}
	sort.Ints(remains)

	result := 0
	for _, remain := range remains {
		if additionalRocks >= remain {
			additionalRocks -= remain
			result += 1
		} else {
			break
		}
	}

	return result
}

type input struct {
	capacity        []int
	rocks           []int
	additionalRocks int
}

func main() {
	inputs := []input{
		{
			capacity:        []int{2, 3, 4, 5},
			rocks:           []int{1, 2, 4, 4},
			additionalRocks: 2,
		},
		{
			capacity:        []int{10, 2, 2},
			rocks:           []int{2, 2, 0},
			additionalRocks: 100,
		},
	}

	for _, input := range inputs {
		result := maximumBags(input.capacity, input.rocks, input.additionalRocks)
		fmt.Println(result)
	}
}
