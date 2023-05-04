package main

import (
	"fmt"
)

func predictPartyVictory(senate string) string {
	const R rune = 'R'
	const D rune = 'D'
	rcount := 0
	dcount := 0
	dfloating := 0
	rfloating := 0

	queue := []rune(senate)
	for _, c := range queue {
		if c == R {
			rcount += 1
		} else {
			dcount += 1
		}
	}

	for (rcount > 0) && (dcount > 0) {
		curr := queue[0]
		queue = queue[1:]
		if curr == D {
			if dfloating > 0 {
				dfloating -= 1
				dcount -= 1
			} else {
				rfloating += 1
				queue = append(queue, D)
			}
		} else {
			if rfloating > 0 {
				rfloating -= 1
				rcount -= 1
			} else {
				dfloating += 1
				queue = append(queue, R)
			}
		}
	}

	if rcount > 0 {
		return "Radiant"
	}

	return "Dire"
}

func main() {
	inputs := []string{
		"RD",
		"RDD",
	}

	for _, senate := range inputs {
		result := predictPartyVictory(senate)
		fmt.Println(result)
	}
}
