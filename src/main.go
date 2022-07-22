package main

import (
	"fmt"
)

type input struct {
	head []int
	x    int
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
			head: []int{1, 4, 3, 2, 5, 2},
			x:    3,
		},
		{
			head: []int{2, 1},
			x:    2,
		},
		{
			head: []int{},
			x:    1,
		},
	}

	for _, input := range inputs {
		result := partition(arrToList(input.head), input.x)
		fmt.Printf("%v\n", listToArr(result))
	}
}
