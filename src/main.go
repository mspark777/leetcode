package main

import (
	"fmt"
)

type input struct {
	root *TreeNode
	q    *TreeNode
	p    *TreeNode
	qv   int
	pv   int
}

func findNode(node *TreeNode, v int) *TreeNode {
	if node == nil {
		return node
	} else if node.Val == v {
		return node
	}

	left := findNode(node.Left, v)
	if left != nil {
		return left
	}

	return findNode(node.Right, v)
}

func main() {
	inputs := []input{
		{
			root: &TreeNode{
				Val: 3,
				Left: &TreeNode{
					Val:  5,
					Left: &TreeNode{Val: 6},
					Right: &TreeNode{
						Val:   2,
						Left:  &TreeNode{Val: 7},
						Right: &TreeNode{Val: 4},
					},
				},
				Right: &TreeNode{
					Val:   1,
					Left:  &TreeNode{Val: 0},
					Right: &TreeNode{Val: 8},
				},
			},
			pv: 5,
			qv: 1,
		},
		{
			root: &TreeNode{
				Val: 3,
				Left: &TreeNode{
					Val:  5,
					Left: &TreeNode{Val: 6},
					Right: &TreeNode{
						Val:   2,
						Left:  &TreeNode{Val: 7},
						Right: &TreeNode{Val: 4},
					},
				},
				Right: &TreeNode{
					Val:   1,
					Left:  &TreeNode{Val: 0},
					Right: &TreeNode{Val: 8},
				},
			},
			pv: 5,
			qv: 4,
		},
		{
			root: &TreeNode{Val: 1, Left: &TreeNode{Val: 2}},
			pv:   1,
			qv:   2,
		},
	}

	for _, input := range inputs {
		input.p = findNode(input.root, input.pv)
		input.q = findNode(input.root, input.qv)
		result := lowestCommonAncestor(input.root, input.p, input.q)
		fmt.Printf("%v\n", result.Val)
	}
}
