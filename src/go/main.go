package main

import (
	"fmt"
)

func isCompleteTree(root *TreeNode) bool {
	if root == nil {
		return true
	}

	nullFound := false
	queue := []*TreeNode{root}

	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]

		if node == nil {
			nullFound = true
			continue
		}

		if nullFound {
			return false
		}

		queue = append(queue, node.Left, node.Right)
	}

	return true
}

func main() {
	inputs := []*TreeNode{
		newTreeNode(1, newTreeNode(2, newTreeVal(4), newTreeVal(5)), newTreeLeft(3, newTreeVal(6))),
		newTreeNode(1, newTreeNode(2, newTreeVal(4), newTreeVal(5)), newTreeRight(3, newTreeVal(7))),
	}

	for _, root := range inputs {
		result := isCompleteTree(root)
		fmt.Println(result)
	}
}
