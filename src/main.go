package main

import (
	"fmt"
)

func countVowelPermutation(n int) int {
	const MOD = 1000000007

	a := 1
	e := 1
	i := 1
	o := 1
	u := 1

	for j := 1; j < n; j += 1 {
		nexta := e + i + u
		nexte := a + i
		nexti := e + o
		nexto := i
		nextu := i + o

		a = nexta % MOD
		e = nexte % MOD
		i = nexti % MOD
		o = nexto % MOD
		u = nextu % MOD
	}

	return (a + e + i + o + u) % MOD
}

type input struct {
	n int
}

func main() {
	inputs := []*input{
		{
			n: 1,
		},
		{
			n: 2,
		},
		{
			n: 5,
		},
		{
			n: 144,
		},
	}

	for _, input := range inputs {
		n := input.n
		result := countVowelPermutation(n)
		fmt.Println(result)
	}
}
