package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func flatten(root *TreeNode) {
	for root != nil {
		if root.Left != nil {
			right := root.Right
			predecessor := root.Left
			for predecessor.Right != nil {
				predecessor = predecessor.Right
			}

			predecessor.Right = right
			root.Right = root.Left
			root.Left = nil
		}

		root = root.Right
	}
}
