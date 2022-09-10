package main

import (
	"fmt"
	"math"
)

type transaction struct {
	spend  int
	profit int
}

func max(i, j int) int {
	if i < j {
		return j
	}

	return i
}

func min(i, j int) int {
	if i < j {
		return i
	}

	return j
}

func maxProfit(k int, prices []int) int {
	if k <= 0 {
		return 0
	}

	transactions := make([]transaction, k+1)
	for i := 0; i <= k; i += 1 {
		transactions[i].spend = math.MaxInt
	}

	for _, price := range prices {
		for i := 1; i <= k; i += 1 {
			prev := &transactions[i-1]
			cur := &transactions[i]

			cur.spend = min(cur.spend, price-prev.profit)
			cur.profit = max(cur.profit, price-cur.spend)
		}
	}

	return transactions[k].profit
}

type input struct {
	k      int
	prices []int
}

func main() {
	inputs := []input{
		{
			k:      2,
			prices: []int{2, 4, 1},
		},
		{
			k:      2,
			prices: []int{3, 2, 6, 5, 0, 3},
		},
	}

	for _, input := range inputs {
		k := input.k
		prices := input.prices
		result := maxProfit(k, prices)
		fmt.Println(result)
	}
}
