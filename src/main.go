package main

import (
	"fmt"
)

type solve struct {
	result [][]int
	temp   []int
}

func (s *solve) solve(i int, candidates []int, target int) {
	if target == 0 {
		t := make([]int, len(s.temp))
		copy(t, s.temp)
		s.result = append(s.result, t)
		return
	}

	if target < 0 {
		return
	}

	if i >= len(candidates) {
		return
	}

	s.solve(i+1, candidates, target)
	candidate := candidates[i]
	s.temp = append(s.temp, candidate)
	s.solve(i, candidates, target-candidate)
	s.temp = s.temp[0 : len(s.temp)-1]
}

func combinationSum(candidates []int, target int) [][]int {
	s := solve{
		result: [][]int{},
		temp:   []int{},
	}
	s.solve(0, candidates, target)
	return s.result
}

type input struct {
	candidates []int
	target     int
}

func main() {
	inputs := []input{
		{candidates: []int{2, 3, 6, 7}, target: 7},
		{candidates: []int{2, 3, 5}, target: 8},
		{candidates: []int{2}, target: 1},
	}

	for _, input := range inputs {
		result := combinationSum(input.candidates, input.target)
		fmt.Println(result)
	}
}
