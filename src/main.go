package main

import (
	"fmt"
	"sort"
)

func findOriginalArray(changed []int) []int {
	result := []int{}
	if (len(changed) % 2) == 1 {
		return result
	}

	queue := []int{}
	head := 0
	sort.Sort(sort.IntSlice(changed))

	for _, i := range changed {
		if len(queue) > head {
			if queue[head] == i {
				head += 1
			} else {
				result = append(result, i)
				queue = append(queue, i*2)
			}
		} else {
			result = append(result, i)
			queue = append(queue, i*2)
		}
	}

	if len(queue) == head {
		return result
	}

	return []int{}
}

type input struct {
	changed []int
}

func main() {
	inputs := []input{
		{
			changed: []int{1, 3, 4, 2, 6, 8},
		},
		{
			changed: []int{6, 3, 0, 1},
		},
		{
			changed: []int{1},
		},
		{
			changed: []int{0, 0, 0, 0},
		},
		{
			changed: []int{6, 3, 0, 1},
		},
		{
			changed: []int{2, 1},
		},
	}

	for _, input := range inputs {
		changed := input.changed
		result := findOriginalArray(changed)
		fmt.Println(result)
	}
}
