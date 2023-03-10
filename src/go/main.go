package main

import (
	"fmt"
	"math/rand"
)

type Solution struct {
	head *ListNode
}

func Constructor(head *ListNode) Solution {
	return Solution{head}
}

func (this *Solution) GetRandom() int {
	scope := float32(1.0)
	result := 0

	for curr := this.head; curr != nil; curr = curr.Next {
		if (rand.Float32() * scope) < 1.0 {
			result = curr.Val
		}

		scope += 1.0
	}

	return result
}

func main() {
	solution := Constructor(newList([]int{1, 2, 3}))
	fmt.Println(solution.GetRandom())
	fmt.Println(solution.GetRandom())
	fmt.Println(solution.GetRandom())
	fmt.Println(solution.GetRandom())
	fmt.Println(solution.GetRandom())
}
