package main

import (
	"fmt"
	"math"
)

func bulbSwitch(n int) int {
	return int(math.Sqrt(float64(n)))
}

func main() {
	inputs := []int{
		3, 0, 1,
	}

	for _, n := range inputs {
		result := bulbSwitch(n)
		fmt.Println(result)
	}
}
