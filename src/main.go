package main

import (
	"fmt"
)

func maxRunTime(n int, batteries []int) int64 {
	sumPower := int64(0)
	for _, power := range batteries {
		sumPower += int64(power)
	}

	left := int64(1)
	right := sumPower / int64(n)

	for left < right {
		target := int64((left + right + 1) / 2)
		extra := int64(0)
		for _, power := range batteries {
			p := int64(power)
			if p < target {
				extra += p
			} else {
				extra += target
			}
		}

		if extra >= (int64(n) * target) {
			left = target
		} else {
			right = target - 1
		}
	}

	return left
}

type input struct {
	n         int
	batteries []int
}

func main() {
	inputs := []input{
		{n: 2, batteries: []int{3, 3, 3}},
		{n: 2, batteries: []int{1, 1, 1, 1}},
	}

	for _, input := range inputs {
		result := maxRunTime(input.n, input.batteries)
		fmt.Println(result)
	}
}
