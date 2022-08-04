package main

import (
	"fmt"
)

func mirrorReflection(p int, q int) int {
	for ((p % 2) + (q % 2)) == 0 {
		p /= 2
		q /= 2
	}

	return (q % 2) - (p % 2) + 1
}

type input struct {
	p int
	q int
}

func main() {
	inputs := []*input{
		{
			p: 2,
			q: 1,
		},
		{
			p: 3,
			q: 1,
		},
	}

	for _, input := range inputs {
		p := input.p
		q := input.q
		result := mirrorReflection(p, q)
		fmt.Println(result)
	}
}
