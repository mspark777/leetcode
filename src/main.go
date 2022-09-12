package main

import (
	"fmt"
	"sort"
)

func bagOfTokensScore(tokens []int, power int) int {
	sort.Sort(sort.IntSlice(tokens))

	score := 0
	result := 0
	i := 0
	j := len(tokens) - 1
	for (i <= j) && ((power >= tokens[i]) || (score > 0)) {
		for (i <= j) && (power >= tokens[i]) {
			power -= tokens[i]
			i += 1
			score += 1
		}

		if score > result {
			result = score
		}

		if (i <= j) && (score > 0) {
			power += tokens[j]
			j -= 1
			score -= 1
		}
	}

	return result
}

type input struct {
	tokens []int
	power  int
}

func main() {
	inputs := []input{
		{
			tokens: []int{100},
			power:  50,
		},
		{
			tokens: []int{100, 200},
			power:  150,
		},
		{
			tokens: []int{100, 200, 300, 400},
			power:  200,
		},
	}

	for _, input := range inputs {
		tokens := input.tokens
		power := input.power
		result := bagOfTokensScore(tokens, power)
		fmt.Println(result)
	}
}
