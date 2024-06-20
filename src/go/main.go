package main

import (
	"fmt"
	"sort"
)

func canPlaceBalls(x int, position []int, m int) bool {
	prevBallPos := position[0]
	ballsPlaced := 1

	for i := range position {
		currPos := position[i]
		if currPos-prevBallPos >= x {
			ballsPlaced += 1
			prevBallPos = currPos
		}

		if ballsPlaced == m {
			return true
		}
	}

	return false
}

func maxDistance(position []int, m int) int {
	answer := 0
	n := len(position)

	sort.Ints(position)
	low := 1
	high := int(float64(position[n-1])/(float64(m)-1.0)) + 1
	for low <= high {
		mid := (low + high) / 2
		if canPlaceBalls(mid, position, m) {
			answer = mid
			low = mid + 1
		} else {
			high = mid - 1
		}
	}
	return answer
}

type input struct {
	position []int
	m        int
}

func main() {
	inputs := []input{
		{
			[]int{1, 2, 3, 4, 7}, 3,
		},
		{
			[]int{5, 4, 3, 2, 1, 1000000000}, 2,
		},
	}

	for _, input := range inputs {
		result := maxDistance(input.position, input.m)
		fmt.Println(result)
	}
}
