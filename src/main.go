package main

import (
	"fmt"
)

type StockSpanner struct {
	stack [][2]int
}

func Constructor() StockSpanner {
	return StockSpanner{stack: [][2]int{}}
}

func (this *StockSpanner) Next(price int) int {
	stack := this.stack
	span := 1

	for len(stack) > 0 {
		topidx := len(stack) - 1
		top := stack[topidx]
		p := top[0]
		s := top[1]

		if p <= price {
			span += s
			stack = stack[:topidx]
		} else {
			break
		}
	}

	this.stack = append(stack, [2]int{price, span})
	return span
}

func main() {
	stockSpanner := Constructor()
	fmt.Println(
		stockSpanner.Next(100),
		stockSpanner.Next(80),
		stockSpanner.Next(60),
		stockSpanner.Next(70),
		stockSpanner.Next(60),
		stockSpanner.Next(75),
		stockSpanner.Next(85),
	)
}
