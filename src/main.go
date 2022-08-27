package main

import "fmt"

func maxSumSubmatrix(matrix [][]int, k int) int {

}

type input struct {
	n int
}

func main() {
	inputs := []input{
		{
			n: 1,
		},
		{
			n: 10,
		},
		{
			n: 46,
		},
	}

	for _, input := range inputs {
		n := input.n
		result := reorderedPowerOf2(n)
		fmt.Println(result)
	}
}
