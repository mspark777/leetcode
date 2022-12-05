package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func middleNode(head *ListNode) *ListNode {
	slow := head
	fast := head

	for (fast != nil) && (fast.Next != nil) {
		slow = slow.Next
		fast = fast.Next.Next
	}

	return slow
}

func arrtolist(nums []int) *ListNode {
	head := ListNode{}
	tail := &head

	for _, val := range nums {
		tail.Next = &ListNode{Val: val}
		tail = tail.Next
	}

	return head.Next
}

func listtoarr(node *ListNode) []int {
	nums := []int{}
	for node != nil {
		nums = append(nums, node.Val)
		node = node.Next
	}

	return nums
}

func main() {
	inputs := [][]int{
		{1, 2, 3, 4, 5},
		{1, 2, 3, 4, 5, 6},
	}

	for _, nums := range inputs {
		result := middleNode(arrtolist(nums))
		fmt.Println(listtoarr(result))
	}
}
