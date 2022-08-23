package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func invertTree(root *TreeNode) *TreeNode {
	stack := []*TreeNode{root}

	for len(stack) > 0 {
		node := stack[0]
		stack = stack[1:]
		if node == nil {
			continue
		}

		left := node.Left
		right := node.Right
		node.Left = right
		node.Right = left
		stack = append(stack, left, right)
	}

	return root
}

type input struct {
	root *TreeNode
}

func createNode(val int, left, right *TreeNode) *TreeNode {
	return &TreeNode{
		Val:   val,
		Left:  left,
		Right: right,
	}
}

func travelInorder(node *TreeNode) {
	if node != nil {
		travelInorder(node.Left)
		fmt.Print(node.Val, " ")
		travelInorder(node.Right)
	}
}

func main() {
	inputs := []input{
		{
			root: createNode(4,
				createNode(2, createNode(1, nil, nil), createNode(3, nil, nil)),
				createNode(7, createNode(6, nil, nil), createNode(9, nil, nil)),
			),
		},
		{
			root: createNode(2,
				createNode(1, nil, nil),
				createNode(3, nil, nil),
			),
		},
		{
			root: nil,
		},
	}

	for _, input := range inputs {
		root := input.root
		result := invertTree(root)
		travelInorder(result)
		fmt.Println()
	}
}
