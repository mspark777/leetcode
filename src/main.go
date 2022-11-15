package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func newnode(val int, left, right *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Left: left, Right: right}
}

func newleft(val int, left *TreeNode) *TreeNode {
	return newnode(val, left, nil)
}

func newright(val int, right *TreeNode) *TreeNode {
	return newnode(val, nil, right)
}

func newval(val int) *TreeNode {
	return newnode(val, nil, nil)
}

func getHeight(root *TreeNode) int {
	if root == nil {
		return -1
	}

	return 1 + getHeight(root.Left)
}

func countNodes(root *TreeNode) int {
	nodes := 0
	h := getHeight(root)
	for root != nil {
		next := h - 1
		if getHeight(root.Right) == next {
			nodes += 1 << h
			root = root.Right
		} else {
			nodes += 1 << next
			root = root.Left
		}
		h = next
	}

	return nodes
}

func main() {
	inputs := []*TreeNode{
		newnode(1, newnode(2, newval(4), newval(5)), newleft(3, newval(6))),
		nil,
		newval(1),
		newnode(1, newleft(2, newval(4)), newval(3)),
	}

	for _, root := range inputs {
		result := countNodes(root)
		fmt.Println(result)
	}
}
