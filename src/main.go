package main

import (
	"fmt"
	"sort"
)

func findWinners(matches [][]int) [][]int {
	loseCounts := map[int]int{}
	for _, match := range matches {
		winner := match[0]
		loser := match[1]

		if _, ok := loseCounts[winner]; !ok {
			loseCounts[winner] = 0
		}

		loseCounts[loser] += 1
	}

	winners := []int{}
	losers := []int{}

	for player, count := range loseCounts {
		if count < 1 {
			winners = append(winners, player)
		} else if count == 1 {
			losers = append(losers, player)
		}
	}

	sort.Ints(winners)
	sort.Ints(losers)

	return [][]int{winners, losers}
}

func main() {
	inputs := [][][]int{
		{{1, 3}, {2, 3}, {3, 6}, {5, 6}, {5, 7}, {4, 5}, {4, 8}, {4, 9}, {10, 4}, {10, 9}},
		{{2, 3}, {1, 3}, {5, 4}, {6, 4}},
	}

	for _, matches := range inputs {
		result := findWinners(matches)
		fmt.Println(result)
	}
}
