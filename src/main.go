package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseRecursively(node *ListNode) *ListNode {
	if node == nil {
		return node
	}

	next := node.Next
	if next == nil {
		return node
	}

	newNode := reverseRecursively(next)
	next.Next = node
	node.Next = nil
	return newNode
}

func reverseIteratively(node *ListNode) *ListNode {
	var prev *ListNode = nil
	for node != nil {
		next := node.Next
		node.Next = prev
		prev = node
		node = next
	}

	return prev
}

func reverseList(head *ListNode) *ListNode {
	return reverseRecursively(head)
}

func arrToList(nums []int) *ListNode {
	dummy := &ListNode{}
	tail := dummy

	for _, num := range nums {
		tail.Next = &ListNode{Val: num}
		tail = tail.Next
	}

	return dummy.Next
}

func listToArr(node *ListNode) []int {
	nums := []int{}

	for node != nil {
		nums = append(nums, node.Val)
		node = node.Next
	}

	return nums
}

type input struct {
	nums []int
}

func main() {
	inputs := []*input{
		{
			nums: []int{1, 2, 3, 4, 5},
		},
		{
			nums: []int{1, 2},
		},
		{
			nums: []int{},
		},
	}

	for _, input := range inputs {
		node := arrToList(input.nums)
		result := reverseList(node)
		fmt.Println(listToArr(result))
	}
}
