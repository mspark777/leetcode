package main

import (
	"fmt"
)

type input struct {
	head  []int
	left  int
	right int
}

func arrToList(arr []int) *ListNode {
	head := &ListNode{}
	tail := head
	for _, v := range arr {
		tail.Next = &ListNode{Val: v}
		tail = tail.Next
	}

	return head.Next
}

func listToArr(node *ListNode) []int {
	arr := []int{}
	for node != nil {
		arr = append(arr, node.Val)
		node = node.Next
	}

	return arr
}

func main() {
	inputs := []input{
		{
			head:  []int{1, 2, 3, 4, 5},
			left:  2,
			right: 4,
		},
		{
			head:  []int{5},
			left:  1,
			right: 1,
		},
		{
			head:  []int{},
			left:  1,
			right: 100,
		},
	}

	for _, input := range inputs {
		result := reverseBetween(
			arrToList(input.head),
			input.left,
			input.right,
		)
		fmt.Printf("%v\n", listToArr(result))
	}
}
