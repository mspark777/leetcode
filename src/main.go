package main

import (
	"fmt"
)

type input struct {
	headA *ListNode
	headB *ListNode
}

func arrtolist(nums []int) *ListNode {
	head := &ListNode{}
	tail := head
	for _, n := range nums {
		tail.Next = &ListNode{Val: n}
		tail = tail.Next
	}

	return head.Next
}

func createInput(numsa []int, numsb []int, skipa int, skipb int) *input {
	headA := arrtolist(numsa[0:skipa])
	headB := arrtolist(numsb[0:skipb])
	if headA == nil || headB == nil {
		return &input{headA: headA, headB: headB}
	}

	remain := arrtolist(numsa[skipa:])

	tail := headA
	for tail != nil && tail.Next != nil {
		tail = tail.Next
	}
	tail.Next = remain

	tail = headB
	for tail != nil && tail.Next != nil {
		tail = tail.Next
	}
	tail.Next = remain

	return &input{headA: headA, headB: headB}
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
	inputs := []*input{
		createInput([]int{4, 1, 8, 4, 5}, []int{5, 6, 1, 8, 4, 5}, 2, 3),
		createInput([]int{1, 9, 1, 2, 4}, []int{3, 2, 4}, 3, 1),
		createInput([]int{2, 6, 4}, []int{1, 5}, 3, 2),
	}

	for _, input := range inputs {
		result := getIntersectionNode(input.headA, input.headB)
		fmt.Printf("%v\n", listtoarr(result))
	}
}
