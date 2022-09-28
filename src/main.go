package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func arrtolist(nums []int) *ListNode {
	dummy := ListNode{}
	tail := &dummy

	for _, num := range nums {
		tail.Next = &ListNode{Val: num}
		tail = tail.Next
	}

	return dummy.Next
}

func listtoarr(node *ListNode) []int {
	nums := []int{}

	for node != nil {
		nums = append(nums, node.Val)
		node = node.Next
	}

	return nums
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	if head == nil {
		return nil
	}

	right := head
	for i := 0; i < n; i += 1 {
		right = right.Next
	}

	if right == nil {
		return head.Next
	}

	left := head
	for right.Next != nil {
		right = right.Next
		left = left.Next
	}

	left.Next = left.Next.Next

	return head
}

type input struct {
	head *ListNode
	n    int
}

func main() {
	inputs := []*input{
		{
			head: arrtolist([]int{1, 2, 3, 4, 5}),
			n:    2,
		}, {
			head: arrtolist([]int{1}),
			n:    1,
		}, {
			head: arrtolist([]int{1, 2}),
			n:    1,
		},
	}

	for _, input := range inputs {
		head := input.head
		n := input.n
		result := removeNthFromEnd(head, n)
		fmt.Println(listtoarr(result))
	}
}
