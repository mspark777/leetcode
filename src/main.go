package main

import (
	"fmt"
)

func change(amount int, coins []int) int {
	dp := make([]int, amount+1)
	dp[0] = 1

	for i := len(coins) - 1; i >= 0; i -= 1 {
		coin := coins[i]
		for j := coin; j <= amount; j += 1 {
			dp[j] += dp[j-coin]
		}
	}

	return dp[amount]
}

type input struct {
	amount int
	coins  []int
}

func main() {
	inputs := []input{
		{amount: 5, coins: []int{1, 2, 5}},
		{amount: 3, coins: []int{2}},
		{amount: 10, coins: []int{10}},
	}

	for _, input := range inputs {
		result := change(input.amount, input.coins)
		fmt.Println(result)
	}
}
