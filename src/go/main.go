package main

import (
	"fmt"
	"sort"
)

func minMovesToSeat(seats []int, students []int) int {
	sort.Ints(seats)
	sort.Ints(students)

	result := 0
	for i, seat := range seats {
		diff := seat - students[i]
		if diff < 0 {
			result -= diff
		} else if diff > 0 {
			result += diff
		}
	}

	return result
}

type input struct {
	seats    []int
	students []int
}

func main() {
	inputs := []input{
		{
			[]int{3, 1, 5},
			[]int{2, 7, 4},
		},
		{
			[]int{4, 1, 5, 9},
			[]int{1, 3, 2, 6},
		},
		{
			[]int{2, 2, 6, 6},
			[]int{1, 3, 2, 6},
		},
	}

	for _, input := range inputs {
		result := minMovesToSeat(input.seats, input.students)
		fmt.Println(result)
	}
}
