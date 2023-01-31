package main

import (
	"fmt"
	"sort"
)

func query(bits []int, age int) int {
	maxScore := -2147483648
	for i := age; i > 0; i -= i & (-i) {
		bit := bits[i]
		if bit > maxScore {
			maxScore = bit
		}
	}

	return maxScore
}

func update(bits []int, age, best int) {
	for i := age; i < len(bits); i += (i & (-i)) {
		bit := bits[i]
		if best > bit {
			bits[i] = best
		}
	}
}

func bestTeamScore(scores []int, ages []int) int {
	pairs := make([][]int, len(ages))
	highestAge := 0
	for i, age := range ages {
		score := scores[i]
		pairs[i] = []int{score, age}

		if age > highestAge {
			highestAge = age
		}
	}

	sort.Slice(pairs, func(i, j int) bool {
		a := pairs[i]
		b := pairs[j]

		if a[0] != b[0] {
			return a[0] < b[0]
		}

		return a[1] < b[1]
	})

	bits := make([]int, highestAge+1)
	result := -2147483648

	for _, pair := range pairs {
		score := pair[0]
		age := pair[1]

		best := score + query(bits, age)
		update(bits, age, best)

		if best > result {
			result = best
		}
	}

	return result
}

func main() {
	inputs := [][][]int{
		{{1, 3, 5, 10, 15}, {1, 2, 3, 4, 5}},
		{{4, 5, 6, 5}, {2, 1, 2, 1}},
		{{1, 2, 3, 5}, {8, 9, 10, 1}},
	}

	for _, input := range inputs {
		result := bestTeamScore(input[0], input[1])
		fmt.Println(result)
	}
}
