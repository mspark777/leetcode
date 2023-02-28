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
