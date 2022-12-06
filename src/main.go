package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func oddEvenList(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	odd := head
	even := odd.Next
	evenHead := even
	for (even != nil) && (even.Next != nil) {
		odd.Next = even.Next
		odd = odd.Next
		even.Next = odd.Next
		even = even.Next
	}

	odd.Next = evenHead
	return head
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
		{2, 1, 3, 5, 6, 4, 7},
	}

	for _, nums := range inputs {
		result := oddEvenList(arrtolist(nums))
		fmt.Println(listtoarr(result))
	}
}
