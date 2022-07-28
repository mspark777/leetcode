package main

import (
	"fmt"
)

type input struct {
	root *TreeNode
}

func main() {
	inputs := []input{
		{
			root: &TreeNode{Val: 1, Right: &TreeNode{Val: 2, Left: &TreeNode{Val: 3}}},
		},
		{
			root: nil,
		},
		{
			root: &TreeNode{Val: 1},
		},
		{
			root: &TreeNode{Val: 1, Left: &TreeNode{Val: 2}},
		},
	}

	for _, input := range inputs {
		result := postorderTraversal(input.root)
		fmt.Printf("%v\n", result)
	}
}
