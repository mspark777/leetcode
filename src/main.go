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
	return newnode(val, nil, right)
}

func newval(val int) *TreeNode {
	return newnode(val, nil, nil)
}

func minDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}

	queue := []*TreeNode{root}
	depth := 0

	for len(queue) > 0 {
		depth += 1
		count := len(queue)

		for i := 0; i < count; i += 1 {
			node := queue[i]
			found := true

			if node.Left != nil {
				found = false
				queue = append(queue, node.Left)
			}

			if node.Right != nil {
				found = false
				queue = append(queue, node.Right)
			}

			if found {
				return depth
			}
		}

		queue = queue[count:]
	}

	return depth
}

func main() {
	inputs := []*TreeNode{
		newnode(3, newval(9), newnode(20, newval(15), newval(7))),
		newright(2, newright(3, newright(4, newright(5, newval(6))))),
	}

	for _, input := range inputs {
		result := minDepth(input)
		fmt.Println(result)
	}
}
