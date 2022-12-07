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

func rangeSumBST(root *TreeNode, low int, high int) int {
	result := 0

	stack := []*TreeNode{root}
	for len(stack) > 0 {
		topidx := len(stack) - 1
		top := stack[topidx]
		stack = stack[:topidx]

		if top == nil {
			continue
		}

		val := top.Val
		left := top.Left
		right := top.Right

		if (low <= val) && (val <= high) {
			result += val
		}

		if low < val {
			stack = append(stack, left)
		}

		if val < high {
			stack = append(stack, right)
		}
	}

	return result
}

type input struct {
	root *TreeNode
	low  int
	high int
}

func main() {
	inputs := []input{
		{
			low:  7,
			high: 15,
			root: newnode(10,
				newnode(5, newval(3), newval(7)),
				newright(15, newval(18)),
			),
		},
		{
			low:  6,
			high: 10,
			root: newnode(10, newnode(5, newleft(3, newval(1)), newleft(7, newval(6))), newnode(5, newval(13), newval(18))),
		},
	}

	for _, input := range inputs {
		root := input.root
		low := input.low
		high := input.high
		result := rangeSumBST(root, low, high)
		fmt.Println(result)
	}
}
