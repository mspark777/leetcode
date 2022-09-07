package main

import (
	"fmt"
	"strings"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func newnode(val int, left, right *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Left: left, Right: right}
}

func newval(val int) *TreeNode {
	return newnode(val, nil, nil)
}

func newleft(val int, left *TreeNode) *TreeNode {
	return newnode(val, left, nil)
}

func newright(val int, right *TreeNode) *TreeNode {
	return newnode(val, nil, right)
}

func tree2str(root *TreeNode) string {
	if root == nil {
		return ""
	}

	stack := []*TreeNode{root}
	visiteds := make(map[*TreeNode]bool)
	result := []string{}

	for len(stack) > 0 {
		topidx := len(stack) - 1
		node := stack[topidx]

		if _, ok := visiteds[node]; ok {
			stack = stack[:topidx]
			result = append(result, ")")
			continue
		}

		visiteds[node] = true
		result = append(result, "(", fmt.Sprint(node.Val))
		left := node.Left
		right := node.Right

		if (left == nil) && (right != nil) {
			result = append(result, "(", ")")
		}

		if right != nil {
			stack = append(stack, right)
		}

		if left != nil {
			stack = append(stack, left)
		}
	}

	return strings.Join(result[1:len(result)-1], "")
}

type input struct {
	root *TreeNode
}

func main() {
	inputs := []input{
		{
			root: newnode(1,
				newleft(2, newval(4)),
				newval(3),
			),
		},
		{
			root: newnode(1,
				newright(2, newval(4)),
				newval(3),
			),
		},
		{
			root: newleft(-1,
				newleft(-2, newleft(-3, newval(-4))),
			),
		},
	}

	for _, input := range inputs {
		root := input.root
		result := tree2str(root)
		fmt.Println(result)
	}
}
