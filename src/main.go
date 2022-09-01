package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func createTreeNode(val int, left *TreeNode, right *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Left: left, Right: right}
}

type stackNode struct {
	node *TreeNode
	max  int
}

func createNode(node *TreeNode, max int) *stackNode {
	return &stackNode{node: node, max: max}
}

func goodNodes(root *TreeNode) int {
	if root == nil {
		return 0
	}

	result := 0

	stack := []*stackNode{createNode(root, root.Val)}
	for len(stack) > 0 {
		topidx := len(stack) - 1
		top := stack[topidx]
		stack = stack[:topidx]

		node := top.node
		val := node.Val
		max := top.max
		if val >= max {
			max = val
			result += 1
		}

		left := node.Left
		if left != nil {
			stack = append(stack, createNode(left, max))
		}

		right := node.Right
		if right != nil {
			stack = append(stack, createNode(right, max))
		}
	}

	return result
}

type input struct {
	root *TreeNode
}

func main() {
	inputs := []input{
		{
			root: createTreeNode(3,
				createTreeNode(1, createTreeNode(3, nil, nil), nil),
				createTreeNode(4, createTreeNode(1, nil, nil), createTreeNode(5, nil, nil)),
			),
		},
		{
			root: createTreeNode(3, createTreeNode(3, createTreeNode(4, nil, nil), createTreeNode(2, nil, nil)), nil),
		},
		{
			root: createTreeNode(1, nil, nil),
		},
	}

	for _, input := range inputs {
		root := input.root
		result := goodNodes(root)
		fmt.Println(result)
	}
}
