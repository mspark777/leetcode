package main

import (
	"fmt"
)

type input struct {
	root *TreeNode
}

func treetoarr(node *TreeNode) []int {
	nums := []int{}
	for node != nil {
		nums = append(nums, node.Val)
		node = node.Right
	}

	return nums
}

func main() {
	inputs := []input{
		{
			root: &TreeNode{
				Val: 1,
				Left: &TreeNode{
					Val:   2,
					Left:  &TreeNode{Val: 3},
					Right: &TreeNode{Val: 4},
				},
				Right: &TreeNode{
					Val:   5,
					Right: &TreeNode{Val: 8},
				},
			},
		},
		{
			root: nil,
		},
		{
			root: &TreeNode{Val: 0},
		},
	}

	for _, input := range inputs {
		flatten(input.root)
		fmt.Printf("%v\n", treetoarr(input.root))
	}
}
