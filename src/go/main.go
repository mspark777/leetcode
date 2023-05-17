package main

import (
	"fmt"
)

func pairSum(head *ListNode) int {
	nodeCount := 0
	for node := head; node != nil; node = node.Next {
		nodeCount += 1
	}

	i := 0
	nums := make([]int, nodeCount)
	for node := head; node != nil; node = node.Next {
		nums[i] = node.Val
		i += 1
	}

	i = 0
	j := nodeCount - 1
	result := 0

	for i < j {
		sum := nums[i] + nums[j]
		if sum > result {
			result = sum
		}

		i += 1
		j -= 1
	}

	return result
}

func main() {
	inputs := []*ListNode{
		newList([]int{5, 4, 2, 1}),
		newList([]int{4, 2, 2, 3}),
		newList([]int{1, 100000}),
	}

	for _, head := range inputs {
		result := pairSum(head)
		fmt.Println(result)
	}
}
