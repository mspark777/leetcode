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

func newval(val int) *TreeNode {
	return newnode(val, nil, nil)
}

func newleft(val int, left *TreeNode) *TreeNode {
	return newnode(val, left, nil)
}

func newright(val int, right *TreeNode) *TreeNode {
	return newnode(val, nil, right)
}

func preorder(node *TreeNode) {
	if node != nil {
		preorder(node.Left)
		fmt.Print(node.Val, " ")
		preorder(node.Right)
	}
}

func containsOne(node *TreeNode) bool {
	if node == nil {
		return false
	}

	leftContained := containsOne(node.Left)
	if !leftContained {
		node.Left = nil
	}

	rightContained := containsOne(node.Right)
	if !rightContained {
		node.Right = nil
	}

	return (node.Val == 1) || leftContained || rightContained
}

func pruneTree(root *TreeNode) *TreeNode {
	if containsOne(root) {
		return root
	}

	return nil
}

type input struct {
	root *TreeNode
}

func main() {
	inputs := []input{
		{
			root: newright(1,
				newnode(0,
					newval(0),
					newval(1),
				),
			),
		},
		{
			root: newnode(1,
				newnode(0,
					newval(0),
					newval(0),
				),
				newnode(1,
					newval(0),
					newval(1),
				),
			),
		},
		{
			root: newnode(1,
				newnode(1,
					newleft(1,
						newval(0),
					),
					newval(1),
				),
				newnode(0,
					newval(0),
					newval(1),
				),
			),
		},
	}

	for _, input := range inputs {
		root := input.root
		result := pruneTree(root)
		preorder(result)
		fmt.Println("")
	}
}
