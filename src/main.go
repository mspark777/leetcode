package main

import (
	"fmt"
)

func isPossible(nums []int) bool {
	lefts := make(map[int]int)
	ends := make(map[int]int)

	for _, num := range nums {
		lefts[num] += 1
	}

	for _, cur := range nums {
		if lefts[cur] == 0 {
			continue
		}

		lefts[cur] -= 1
		before1 := cur - 1
		if ends[before1] > 0 {
			ends[before1] -= 1
			ends[cur] += 1
			continue
		}

		after1 := cur + 1
		after2 := cur + 2
		if (lefts[after1]) > 0 && (lefts[after2] > 0) {
			lefts[after1] -= 1
			lefts[after2] -= 1
			ends[after2] += 1
			continue
		}

		return false
	}

	return true
}

type input struct {
	nums []int
}

func main() {
	inputs := []*input{
		{
			nums: []int{1, 2, 3, 3, 4, 5},
		},
		{
			nums: []int{1, 2, 3, 3, 4, 4, 5, 5},
		},
		{
			nums: []int{1, 2, 3, 4, 4, 5},
		},
	}

	for _, input := range inputs {
		nums := input.nums
		result := isPossible(nums)
		fmt.Println(result)
	}
}
