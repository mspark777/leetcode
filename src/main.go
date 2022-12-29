package main

import (
	"fmt"
)

type NumArray struct {
	sums []int
}

func Constructor(nums []int) NumArray {
	sums := make([]int, len(nums)+1)
	for i, num := range nums {
		sums[i+1] += sums[i] + num
	}

	return NumArray{sums}
}

func (this *NumArray) SumRange(left int, right int) int {
	sums := this.sums
	return sums[right+1] - sums[left]
}

func main() {
	obj := Constructor([]int{-2, 0, 3, -5, 2, -1})
	fmt.Println(obj.SumRange(0, 2))
	fmt.Println(obj.SumRange(2, 5))
	fmt.Println(obj.SumRange(0, 5))
}
