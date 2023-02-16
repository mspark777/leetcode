package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func newnode(val int, left, right *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Left: left, Right: right}
}

func newright(val int, right *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Right: right}
}

func newval(val int) *TreeNode {
	return &TreeNode{Val: val}
}

func travel(node *TreeNode, depth int, maxDepth *int) {
	if node != nil {
		d := depth + 1
		travel(node.Left, d, maxDepth)
		travel(node.Right, d, maxDepth)
	} else {
		if depth > *maxDepth {
			*maxDepth = depth
		}
	}
}

func maxDepth(root *TreeNode) int {
	result := 0
	travel(root, 0, &result)

	return result
}

func main() {
	inputs := []*TreeNode{
		newnode(3, newval(9), newnode(20, newval(15), newval(7))),
		newright(1, newval(2)),
	}

	for _, input := range inputs {
		result := maxDepth(input)
		fmt.Println(result)
	}
}
