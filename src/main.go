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

func findTarget(root *TreeNode, k int) bool {
	stack := []*TreeNode{root}
	memo := map[int]bool{}

	for len(stack) > 0 {
		topidx := len(stack) - 1
		top := stack[topidx]
		stack = stack[:topidx]

		if top == nil {
			continue
		}

		val := top.Val
		target := k - val
		if _, ok := memo[target]; ok {
			return true
		} else {
			memo[val] = true
		}

		stack = append(stack, top.Left, top.Right)
	}

	return false
}

type input struct {
	root *TreeNode
	k    int
}

func main() {
	inputs := []input{
		{
			root: newnode(5, newnode(3, newval(2), newval(4)), newright(6, newval(7))),
			k:    9,
		},
		{
			root: newnode(5, newnode(3, newval(2), newval(4)), newright(6, newval(7))),
			k:    28,
		},
	}

	for _, input := range inputs {
		root := input.root
		k := input.k
		result := findTarget(root, k)
		fmt.Println(result)
	}
}
