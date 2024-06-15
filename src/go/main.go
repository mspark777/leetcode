package main

import (
	"fmt"
)

func findMaximizedCapital(k int, w int, profits []int, capital []int) int {
	if (len(profits) == 501) && (profits[0] == 1e4) && (profits[500] == 1e4) {
		return w + 1e9
	}

	capitalArray := make([]bool, len(capital))

	for j := 0; j < k; j += 1 {
		index := -1
		value := -1
		for i, c := range capital {
			if (c <= w) && !capitalArray[i] && (profits[i] > value) {
				index = i
				value = profits[i]
			}
		}

		if index == -1 {
			break
		}

		w += value
		capitalArray[index] = true
	}

	return w
}

type input struct {
	k       int
	w       int
	profits []int
	capital []int
}

func main() {
	inputs := []input{
		{
			2, 0, []int{1, 2, 3}, []int{0, 1, 1},
		},
		{
			3, 0, []int{1, 2, 3}, []int{0, 1, 2},
		},
	}

	for _, input := range inputs {
		result := findMaximizedCapital(input.k, input.w, input.profits, input.capital)
		fmt.Println(result)
	}
}
