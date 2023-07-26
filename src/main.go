package main

import (
	"fmt"
	"math"
)

func timeRequired(dist []int, speed int) float64 {
	time := 0.0
	for i, d := range dist {
		t := float64(d) / float64(speed)
		if i == (len(dist) - 1) {
			time += t
		} else {
			time += math.Ceil(t)
		}
	}

	return time
}

func minSpeedOnTime(dist []int, hour float64) int {
	left := 1
	right := 10000001
	minSpeed := -1

	for left <= right {
		mid := (left + right) / 2
		if timeRequired(dist, mid) <= hour {
			minSpeed = mid
			right = mid - 1
		} else {
			left = mid + 1
		}
	}

	return minSpeed
}

type input struct {
	dist []int
	hour float64
}

func main() {
	inputs := []input{
		{dist: []int{1, 3, 2}, hour: 6},
		{dist: []int{1, 3, 2}, hour: 2.7},
		{dist: []int{1, 3, 2}, hour: 1.9},
		{dist: []int{1, 1, 100000}, hour: 2.01},
		{dist: []int{90, 94, 72, 85, 92, 63, 20, 25, 38, 28, 8, 75, 95, 70, 8, 96, 41, 8, 7, 75, 62, 65, 68, 21, 8, 66, 11, 24, 9, 77, 9, 87, 31, 52, 16, 73, 32, 75, 77, 6, 80, 11, 54, 85, 75, 73, 67, 41, 34, 27, 86, 92, 41, 31, 34, 51, 17, 86, 83, 39, 59, 41, 97, 10, 2, 59, 80, 73, 13, 10, 69, 28, 65, 34, 17, 45, 9}, hour: 393.18},
	}

	for _, input := range inputs {
		result := minSpeedOnTime(input.dist, input.hour)
		fmt.Println(result)
	}
}
