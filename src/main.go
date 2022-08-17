package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeElements(head *ListNode, val int) *ListNode {
	dummy := &ListNode{Val: -1, Next: head}
	cur := dummy
	for cur != nil {
		for (cur.Next != nil) && (cur.Next.Val == val) {
			cur.Next = cur.Next.Next
		}

		cur = cur.Next
	}

	return dummy.Next
}

type input struct {
	head *ListNode
	val  int
}

func arrToList(nums []int) *ListNode {
	head := &ListNode{}
	tail := head

	for _, num := range nums {
		tail.Next = &ListNode{Val: num}
		tail = tail.Next
	}

	return head.Next
}

func listToArr(node *ListNode) []int {
	nums := []int{}
	for node != nil {
		nums = append(nums, node.Val)
		node = node.Next
	}

	return nums
}

func main() {
	inputs := []*input{
		{
			val:  6,
			head: arrToList([]int{1, 2, 6, 3, 4, 5, 6}),
		},
		{
			val:  1,
			head: arrToList([]int{}),
		},
		{
			val:  7,
			head: arrToList([]int{7, 7, 7, 7}),
		},
	}

	for _, input := range inputs {
		head := input.head
		val := input.val
		result := removeElements(head, val)
		fmt.Println(listToArr(result))
	}
}
