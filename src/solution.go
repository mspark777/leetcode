package main

import "math"

func maxProfit(prices []int) int {
	minPrice := math.MaxInt
	maxProfit := 0

	for _, price := range prices {
		if minPrice > price {
			minPrice = price
		}

		profit := price - minPrice
		if maxProfit < profit {
			maxProfit = profit
		}
	}

	return maxProfit
}
