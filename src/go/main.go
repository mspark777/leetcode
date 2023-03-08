package main

import (
	"fmt"
)

func hoursRequired(piles []int, k int) int {
	if k == 0 {
		return int((^uint(0)) >> 1)
	}

	hours := 0
	for _, pile := range piles {
		if (pile % k) != 0 {
			hours += 1
		}

		hours += pile / k
	}

	return hours
}

func minEatingSpeed(piles []int, h int) int {
	sum := 0
	maxPile := 0

	for _, pile := range piles {
		sum += pile
		if pile > maxPile {
			maxPile = pile
		}
	}

	left := sum / h
	right := maxPile
	for left < right {
		middle := (left + right) / 2
		required := hoursRequired(piles, middle)
		if required > h {
			left = middle + 1
		} else {
			right = middle
		}
	}

	return left
}

type input struct {
	piles []int
	h     int
}

func main() {
	inputs := []input{
		{piles: []int{3, 6, 7, 11}, h: 8},
		{piles: []int{30, 11, 23, 4, 20}, h: 5},
		{piles: []int{30, 11, 23, 4, 20}, h: 6},

		{piles: []int{332484035, 524908576, 855865114, 632922376, 222257295,
			690155293, 112677673, 679580077, 337406589, 290818316,
			877337160, 901728858, 679284947, 688210097, 692137887,
			718203285, 629455728, 941802184}, h: 823855818},
	}

	for _, input := range inputs {
		result := minEatingSpeed(input.piles, input.h)
		fmt.Println(result)
	}
}
