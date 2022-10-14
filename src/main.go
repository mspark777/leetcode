package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func arrlist(nums []int) *ListNode {
	temp := &ListNode{}
	tail := temp

	for _, num := range nums {
		tail.Next = &ListNode{Val: num}
		tail = tail.Next
	}

	return temp.Next
}

func listarr(node *ListNode) []int {
	nums := []int{}

	for node != nil {
		nums = append(nums, node.Val)
		node = node.Next
	}

	return nums
}

func deleteMiddle(head *ListNode) *ListNode {
	if (head == nil) || (head.Next == nil) {
		return nil
	}

	slow := head
	fast := head.Next.Next

	for (fast != nil) && (fast.Next != nil) {
		slow = slow.Next
		fast = fast.Next.Next
	}

	slow.Next = slow.Next.Next
	return head
}

func main() {
	inputs := [][]int{
		{1, 3, 4, 7, 1, 2, 6},
		{1, 2, 3, 4},
		{2, 1},
	}

	for _, nums := range inputs {
		result := deleteMiddle(arrlist(nums))
		fmt.Println(listarr(result))
	}
}
