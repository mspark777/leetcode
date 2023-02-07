package main

import (
	"fmt"
)

func totalFruit(fruits []int) int {
	baskets := map[int]int{}
	left := 0
	result := 0

	for right, rfruit := range fruits {
		baskets[rfruit] += 1

		for len(baskets) > 2 {
			lfruit := fruits[left]
			baskets[lfruit] -= 1
			if cnt := baskets[lfruit]; cnt == 0 {
				delete(baskets, lfruit)
			}

			left += 1
		}

		cnt := right - left + 1
		if cnt > result {
			result = cnt
		}
	}

	return result
}

func main() {
	inputs := [][]int{
		{1, 2, 1},
		{0, 1, 2, 2},
		{1, 2, 3, 2, 2},
		{3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4},
	}

	for _, input := range inputs {
		result := totalFruit(input)
		fmt.Println(result)
	}
}
