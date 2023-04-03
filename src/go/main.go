package main

import (
	"fmt"
	"sort"
)

func numRescueBoats(people []int, limit int) int {
	sort.Ints(people)
	left := 0
	right := len(people) - 1
	result := 0

	for left <= right {
		light := people[left]
		heavy := people[right]
		total := light + heavy

		result += 1
		right -= 1
		if total <= limit {
			left += 1
		}
	}

	return result
}

type input struct {
	people []int
	limit  int
}

func main() {
	inputs := []input{
		{[]int{1, 2}, 3},
		{[]int{3, 2, 2, 1}, 3},
		{[]int{3, 5, 3, 4}, 5},
	}

	for _, input := range inputs {
		result := numRescueBoats(input.people, input.limit)
		fmt.Println(result)
	}
}
