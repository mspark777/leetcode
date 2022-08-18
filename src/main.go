package main

import (
	"fmt"
	"sort"
)

func minSetSize(arr []int) int {
	freqs := make(map[int]int)
	for _, num := range arr {
		freqs[num] += 1
	}

	pqueue := []int{}
	for _, freq := range freqs {
		pqueue = append(pqueue, freq)
	}
	sort.Sort(sort.Reverse(sort.IntSlice(pqueue)))

	deleted := 0
	result := 0
	half := len(arr) / 2
	for _, freq := range pqueue {
		deleted += freq
		result += 1
		if deleted >= half {
			return result
		}
	}

	return -1
}

type input struct {
	arr []int
}

func main() {
	inputs := []*input{
		{
			arr: []int{3, 3, 3, 3, 5, 5, 5, 2, 2, 7},
		},
		{
			arr: []int{7, 7, 7, 7, 7, 7},
		},
	}

	for _, input := range inputs {
		arr := input.arr
		result := minSetSize(arr)
		fmt.Println(result)
	}
}
