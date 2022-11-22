package main

import (
	"fmt"
)

func numSquares(n int) int {
	memos := make([]int, n+1)
	for i := 1; i <= n; i += 1 {
		memos[i] = 2147483647
	}

	cur := 1
	squire := 1

	for squire <= n {
		for i := squire; i <= n; i += 1 {
			to := memos[i]
			from := memos[i-squire] + 1
			if from < to {
				memos[i] = from
			}

		}

		cur += 1
		squire = cur * cur
	}

	return memos[n]
}

func main() {
	inputs := []int{
		12, 13,
	}

	for _, n := range inputs {
		result := numSquares(n)
		fmt.Println(result)
	}
}
