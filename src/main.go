package main

import (
	"fmt"
	"sort"
)

func numberOfWeakCharacters(properties [][]int) int {
	sort.Slice(properties, func(i, j int) bool {
		a := properties[i]
		b := properties[j]

		attackA := a[0]
		defenceA := a[1]

		attackB := b[0]
		defenceB := b[1]

		if attackA == attackB {
			return defenceA < defenceB
		}

		return attackB < attackA
	})

	maxDefence := 0
	result := 0

	for _, p := range properties {
		defence := p[1]
		if maxDefence > defence {
			result += 1
		} else {
			maxDefence = defence
		}
	}

	return result
}

type input struct {
	properties [][]int
}

func main() {
	inputs := []input{
		{
			properties: [][]int{{5, 5}, {6, 3}, {3, 6}},
		},
		{
			properties: [][]int{{2, 2}, {3, 3}},
		},
		{
			properties: [][]int{{1, 5}, {10, 4}, {4, 3}},
		},
	}

	for _, input := range inputs {
		properties := input.properties
		result := numberOfWeakCharacters(properties)
		fmt.Println(result)
	}
}
