package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func newnode(val int, left, right *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Left: left, Right: right}
}

func newval(val int) *TreeNode {
	return newnode(val, nil, nil)
}

func newleft(val int, left *TreeNode) *TreeNode {
	return newnode(val, left, nil)
}

func newright(val int, right *TreeNode) *TreeNode {
	return newnode(val, nil, right)
}

func inorderTraversal(root *TreeNode) []int {
	stack := []*TreeNode{}
	result := []int{}
	top := root

	for (top != nil) || (len(stack) > 0) {
		for top != nil {
			stack = append(stack, top)
			top = top.Left
		}

		topidx := len(stack) - 1
		top = stack[topidx]
		stack = stack[:topidx]
		result = append(result, top.Val)
		top = top.Right
	}

	return result
}

type input struct {
	root *TreeNode
}

func main() {
	inputs := []input{
		{
			root: newright(1, newleft(2, newval(3))),
		},
		{
			root: nil,
		},
		{
			root: newval(1),
		},
	}

	for _, input := range inputs {
		root := input.root
		result := inorderTraversal(root)
		fmt.Println(result)
	}
}
