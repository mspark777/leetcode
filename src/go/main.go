package main

import (
	"fmt"
)

func swapPairs(head *ListNode) *ListNode {
	if head == nil {
		return head
	} else if head.Next == nil {
		return head
	}

	dummy := new(ListNode)
	prev := dummy
	curr := head

	for (curr != nil) && (curr.Next != nil) {
		prev.Next = curr.Next
		curr.Next = prev.Next.Next
		prev.Next.Next = curr

		prev = curr
		curr = curr.Next
	}

	return dummy.Next
}

func main() {
	inputs := []*ListNode{
		newList([]int{1, 2, 3, 4}),
		newList([]int{}),
		newList([]int{1}),
	}

	for _, input := range inputs {
		result := swapPairs(input)
		fmt.Println(listToArr(result))
	}
}
