package main

import (
	"fmt"
)

func swapNodes(head *ListNode, k int) *ListNode {
	left := head
	for i := 1; i < k; i += 1 {
		if left != nil {
			left = left.Next
		}
	}

	if left == nil {
		return head
	}

	right := head
	for i := left; i.Next != nil; i = i.Next {
		right = right.Next
	}

	if right == nil {
		return head
	}

	l := left.Val
	r := right.Val

	left.Val = r
	right.Val = l

	return head
}

type input struct {
	head *ListNode
	k    int
}

func main() {
	inputs := []input{
		{head: newList([]int{1, 2, 3, 4, 5}), k: 2},
		{head: newList([]int{7, 9, 6, 6, 7, 8, 3, 0, 9, 5}), k: 5},
	}

	for _, input := range inputs {
		result := swapNodes(input.head, input.k)
		fmt.Println(listToArr(result))
	}
}
