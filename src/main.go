package main

import (
	"fmt"
)

type input struct {
	head []int
	pos  int
}

func arrtolist(nums []int, pos int) *ListNode {
	head := &ListNode{}
	tail := head
	var cycle *ListNode = nil
	for i, v := range nums {
		node := &ListNode{Val: v}
		tail.Next = node
		tail = node
		if i == pos {
			cycle = node
		}
	}

	tail.Next = cycle
	return head.Next
}

func main() {
	inputs := []input{
		{
			head: []int{3, 2, 0, -4},
			pos:  1,
		},
		{
			head: []int{1, 2},
			pos:  0,
		},
		{
			head: []int{1},
			pos:  -1,
		},
	}

	for _, input := range inputs {
		result := hasCycle(arrtolist(input.head, input.pos))
		fmt.Printf("%v\n", result)
	}
}
