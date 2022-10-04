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

type stackNode struct {
	node   *TreeNode
	target int
}

func hasPathSum(root *TreeNode, targetSum int) bool {
	if root == nil {
		return false
	}

	stack := []*stackNode{{node: root, target: targetSum}}

	for len(stack) > 0 {
		topidx := len(stack) - 1
		top := stack[topidx]
		stack = stack[:topidx]

		node := top.node
		target := top.target
		newval := target - node.Val
		left := node.Left
		right := node.Right
		isleaf := true

		if left != nil {
			isleaf = false
			stack = append(stack, &stackNode{node: left, target: newval})
		}

		if right != nil {
			isleaf = false
			stack = append(stack, &stackNode{node: right, target: newval})
		}

		if isleaf && (newval == 0) {
			return true
		}
	}

	return false
}

type input struct {
	root      *TreeNode
	targetSum int
}

func main() {
	inputs := []input{
		{
			root:      newnode(5, newleft(4, newnode(11, newval(7), newval(2))), newnode(8, newval(13), newright(4, newval(1)))),
			targetSum: 22,
		},
		{
			root:      newnode(5, newval(2), newval(3)),
			targetSum: 5,
		},
		{
			root:      nil,
			targetSum: 0,
		},
	}

	for _, input := range inputs {
		root := input.root
		targetSum := input.targetSum
		result := hasPathSum(root, targetSum)
		fmt.Println(result)
	}
}
