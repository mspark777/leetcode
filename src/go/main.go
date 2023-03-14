package main

import (
	"fmt"
)

func travel(node *TreeNode, sum int) int {
	if node == nil {
		return 0
	}

	newsum := (sum * 10) + node.Val
	left := node.Left
	right := node.Right

	if (left == nil) && (right == nil) {
		return newsum
	}

	return travel(left, newsum) + travel(right, newsum)
}

func sumNumbers(root *TreeNode) int {
	return travel(root, 0)
}

func main() {
	inputs := []*TreeNode{
		newTreeNode(1, newTreeVal(2), newTreeVal(3)),
		newTreeNode(4, newTreeNode(9, newTreeVal(5), newTreeVal(1)), newTreeVal(0)),
	}

	for _, root := range inputs {
		result := sumNumbers(root)
		fmt.Println(result)
	}
}
