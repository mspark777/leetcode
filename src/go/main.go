package main

import "fmt"

func subtractProductAndSum(n int) int {
	mul := 1
	sum := 0
	for n > 0 {
		d := n % 10
		n /= 10

		mul *= d
		sum += d
	}

	return mul - sum
}

type input struct {
	n int
}

func main() {
	inputs := []input{
		{
			n: 234,
		},
		{
			n: 4421,
		},
	}

	for _, input := range inputs {
		result := subtractProductAndSum(input.n)
		fmt.Println(result)
	}
}
