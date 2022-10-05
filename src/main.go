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

func treetoarr(node *TreeNode, nums []int) []int {
	if node != nil {
		nums = append(nums, node.Val)
		nums = treetoarr(node.Left, nums)
		nums = treetoarr(node.Right, nums)
	}

	return nums
}

type stackNode struct {
	node *TreeNode
	pos  int
}

func addOneRow(root *TreeNode, val int, depth int) *TreeNode {
	if root == nil {
		return nil
	}

	if depth == 1 {
		return &TreeNode{Val: val, Left: root}
	}

	stack := []*stackNode{{node: root, pos: 2}}
	targets := []*TreeNode{}

	for len(stack) > 0 {
		topidx := len(stack) - 1
		top := stack[topidx]
		stack = stack[:topidx]

		pos := top.pos
		node := top.node

		if pos > depth {
			continue
		}

		if pos == depth {
			targets = append(targets, node)
		}

		left := node.Left
		if left != nil {
			stack = append(stack, &stackNode{node: left, pos: pos + 1})
		}

		right := node.Right
		if right != nil {
			stack = append(stack, &stackNode{node: right, pos: pos + 1})
		}
	}

	for _, target := range targets {
		target.Left = &TreeNode{Val: val, Left: target.Left}
		target.Right = &TreeNode{Val: val, Right: target.Right}
	}

	return root
}

type input struct {
	root  *TreeNode
	val   int
	depth int
}

func main() {
	inputs := []input{
		{
			root:  newnode(4, newnode(2, newval(3), newval(1)), newleft(6, newval(5))),
			val:   1,
			depth: 2,
		},
		{
			root:  newleft(4, newnode(2, newval(3), newval(1))),
			val:   1,
			depth: 3,
		},
		{
			root:  newnode(4, newnode(2, newval(3), newval(1)), newleft(6, newval(5))),
			val:   1,
			depth: 1,
		},
	}

	for _, input := range inputs {
		root := input.root
		val := input.val
		depth := input.depth
		result := addOneRow(root, val, depth)
		fmt.Println(treetoarr(result, []int{}))
	}
}
