package main

import "fmt"

func getCounts(n int) []int {
	result := make([]int, 10)
	for n > 0 {
		result[n%10] += 1
		n /= 10
	}

	return result
}

func compareCounts(c1 []int, c2 []int) bool {
	for i, c := range c1 {
		if c != c2[i] {
			return false
		}
	}

	return true
}

func reorderedPowerOf2(n int) bool {
	counts := getCounts(n)
	for i := 0; i < 31; i += 1 {
		if compareCounts(counts, getCounts(1<<i)) {
			return true
		}
	}

	return false
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
