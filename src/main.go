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
	return newnode(val, left, nil)
}

func newright(val int, right *TreeNode) *TreeNode {
	return newnode(val, nil, right)
}

func newval(val int) *TreeNode {
	return newnode(val, nil, nil)
}

func isSameTree(p *TreeNode, q *TreeNode) bool {
	if (p == nil) && (q == nil) {
		return true
	}

	if (p == nil) || (q == nil) {
		return false
	}

	if p.Val != q.Val {
		return false
	}

	return isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}

func main() {
	inputs := [][]*TreeNode{
		{
			newnode(1, newval(2), newval(3)),
			newnode(1, newval(2), newval(3)),
		},
		{
			newleft(1, newval(2)),
			newright(1, newval(2)),
		},
		{
			newnode(1, newval(2), newval(1)),
			newnode(1, newval(1), newval(2)),
		},
	}

	for _, input := range inputs {
		result := isSameTree(input[0], input[1])
		fmt.Println(result)
	}
}
