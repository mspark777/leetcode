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
			root: &TreeNode{
				Val: 1,
				Right: &TreeNode{
					Val:  2,
					Left: &TreeNode{Val: 3},
				},
			},
		},
		{
			root: nil,
		},
		{
			root: &TreeNode{Val: 1},
		},
	}

	for _, input := range inputs {
		result := preorderTraversal(input.root)
		fmt.Printf("%v\n", result)
	}
}
