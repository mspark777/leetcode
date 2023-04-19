package main

import (
	"fmt"
)

func dfs(node *TreeNode, left bool, steps int, maxSteps *int) {
	if node == nil {
		return
	}

	if steps > *maxSteps {
		*maxSteps = steps
	}

	if left {
		dfs(node.Left, false, steps+1, maxSteps)
		dfs(node.Right, true, 1, maxSteps)
	} else {
		dfs(node.Left, false, 1, maxSteps)
		dfs(node.Right, true, steps+1, maxSteps)
	}
}

func longestZigZag(root *TreeNode) int {
	maxSteps := 0
	dfs(root, true, 0, &maxSteps)
	dfs(root, false, 0, &maxSteps)

	return maxSteps
}

func main() {
	inputs := []*TreeNode{
		newTreeRight(1, newTreeNode(1, newTreeVal(1), newTreeNode(1, newTreeRight(1, newTreeRight(1, newTreeVal(1))), newTreeVal(1)))),
		newTreeNode(1, newTreeRight(1, newTreeNode(1, newTreeRight(1, newTreeVal(1)), newTreeVal(1))), newTreeVal(1)),
		newTreeVal(1),
	}

	for _, root := range inputs {
		result := longestZigZag(root)
		fmt.Println(result)
	}
}
