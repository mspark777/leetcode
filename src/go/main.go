package main

import (
	"fmt"
)

func canPlaceFlowers(flowerbed []int, n int) bool {
	count := 0
	last := len(flowerbed) - 1
	for i, plot := range flowerbed {
		if plot == 1 {
			continue
		}

		emptyLeft := (i == 0) || (flowerbed[i-1] == 0)
		emptyRight := (i == last) || (flowerbed[i+1] == 0)
		if emptyLeft && emptyRight {
			flowerbed[i] = 1
			count += 1
			if count >= n {
				return true
			}
		}
	}

	return count >= n
}

type input struct {
	flowerbed []int
	n         int
}

func main() {
	inputs := []input{
		{flowerbed: []int{1, 0, 0, 0, 1}, n: 1},
		{flowerbed: []int{1, 0, 0, 0, 1}, n: 2},
	}

	for _, input := range inputs {
		result := canPlaceFlowers(input.flowerbed, input.n)
		fmt.Println(result)
	}
}
