package main

import (
	"fmt"
)

type queueNode struct {
	node  *TreeNode
	index int
}

func widthOfBinaryTree(root *TreeNode) int {
	if root == nil {
		return 0
	}

	result := 1
	queue := []*queueNode{{root, 0}}

	for len(queue) > 0 {
		count := len(queue)
		start := queue[0].index
		end := queue[count-1].index
		current := end - start + 1
		if current > result {
			result = current
		}

		for i := 0; i < count; i += 1 {
			qnode := queue[i]
			node := qnode.node
			nodeIdx := qnode.index
			idx := nodeIdx - start
			left := node.Left
			right := node.Right

			if left != nil {
				queue = append(queue, &queueNode{left, 2*idx + 1})
			}

			if right != nil {
				queue = append(queue, &queueNode{right, 2 * (idx + 1)})
			}
		}

		queue = queue[count:]
	}

	return result
}

func main() {
	inputs := []*TreeNode{
		newTreeNode(1, newTreeNode(3, newTreeVal(5), newTreeVal(3)), newTreeRight(2, newTreeVal(9))),
		newTreeNode(1, newTreeLeft(3, newTreeLeft(5, newTreeVal(6))), newTreeRight(2, newTreeLeft(9, newTreeVal(7)))),
		newTreeNode(1, newTreeLeft(3, newTreeVal(5)), newTreeVal(2)),
	}

	for _, root := range inputs {
		result := widthOfBinaryTree(root)
		fmt.Println(result)
	}
}
