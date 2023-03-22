package main

import (
	"fmt"
)

func minScore(n int, roads [][]int) int {

}

func main() {
	inputs := [][]int{
		{1, 3, 0, 0, 2, 0, 0, 4},
		{0, 0, 0, 2, 0, 0},
		{2, 10, 2019},
	}

	for _, nums := range inputs {
		result := zeroFilledSubarray(nums)
		fmt.Println(result)
	}
}
