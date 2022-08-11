package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isValidBST(root *TreeNode) bool {
	if root == nil {
		return true
	}

	var pre *TreeNode = nil
	stack := []*TreeNode{}

	for (root != nil) || (len(stack) > 0) {
		for root != nil {
			stack = append(stack, root)
			root = root.Left
		}

		top := len(stack) - 1
		root = stack[top]
		stack = stack[:top]

		if (pre != nil) && (root.Val <= pre.Val) {
			return false
		}

		pre = root
		root = root.Right
	}

	return true
}

type input struct {
	root *TreeNode
}

func main() {
	inputs := []*input{
		{
			root: &TreeNode{
				Val:   2,
				Left:  &TreeNode{Val: 1},
				Right: &TreeNode{Val: 3},
			},
		},
		{
			root: &TreeNode{
				Val:  4,
				Left: &TreeNode{Val: 1},
				Right: &TreeNode{
					Val:   4,
					Left:  &TreeNode{Val: 3},
					Right: &TreeNode{Val: 6},
				},
			},
		},
	}

	for _, input := range inputs {
		root := input.root
		result := isValidBST(root)
		fmt.Println(result)
	}
}
