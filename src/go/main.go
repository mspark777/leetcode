package main

import (
	"fmt"
)

func isMirror(left, right *TreeNode) bool {
	if (left == nil) && (right == nil) {
		return true
	} else if (left == nil) || (right == nil) {
		return false
	}

	return (left.Val == right.Val) &&
		isMirror(left.Left, right.Right) &&
		isMirror(left.Right, right.Left)
}

func isSymmetric(root *TreeNode) bool {
	if root == nil {
		return true
	}

	return isMirror(root.Left, root.Right)
}

func main() {
	inputs := []*TreeNode{
		newTreeNode(1, newTreeNode(2, newTreeVal(3), newTreeVal(4)), newTreeNode(2, newTreeVal(4), newTreeVal(3))),
		newTreeNode(1, newTreeRight(2, newTreeVal(3)), newTreeRight(2, newTreeVal(3))),
	}

	for _, root := range inputs {
		result := isSymmetric(root)
		fmt.Println(result)
	}
}
