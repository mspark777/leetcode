package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func newTreeNode(val int, left, right *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Left: left, Right: right}
}

func newTreeLeft(val int, left *TreeNode) *TreeNode {
	return newTreeNode(val, left, nil)
}

func newTreeRight(val int, right *TreeNode) *TreeNode {
	return newTreeNode(val, nil, right)
}

func newTreeVal(val int) *TreeNode {
	return newTreeNode(val, nil, nil)
}

type ListNode struct {
	Val  int
	Next *ListNode
}

func newListNode(val int, next *ListNode) *ListNode {
	return &ListNode{Val: val, Next: next}
}

func newCycleList(vals []int, pos int) *ListNode {
	head := newListNode(0, nil)
	tail := head
	var target *ListNode = nil
	for i, val := range vals {
		node := newListNode(val, nil)
		if i == pos {
			target = node
		}

		tail.Next = node
		tail = node
	}

	tail.Next = target
	return head.Next
}
