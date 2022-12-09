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

func newleft(val int, left *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Left: left}
}

func newright(val int, right *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Right: right}
}

func newval(val int) *TreeNode {
	return &TreeNode{Val: val}
}

func travel(node *TreeNode, curmax, curmin int) int {
	if node == nil {
		return curmax - curmin
	}

	val := node.Val
	if val > curmax {
		curmax = val
	}

	if val < curmin {
		curmin = val
	}

	left := travel(node.Left, curmax, curmin)
	right := travel(node.Right, curmax, curmin)
	if left > right {
		return left
	}

	return right
}

func maxAncestorDiff(root *TreeNode) int {
	if root == nil {
		return 0
	}

	return travel(root, root.Val, root.Val)
}

func main() {
	inputs := []*TreeNode{
		newnode(8,
			newnode(3,
				newval(1),
				newnode(6,
					newval(4),
					newval(7),
				),
			),
			newright(10,
				newleft(14,
					newval(13),
				),
			),
		),
		newright(1,
			newright(2,
				newleft(0,
					newval(3),
				),
			),
		),
	}

	for _, root := range inputs {
		result := maxAncestorDiff(root)
		fmt.Println(result)
	}
}
