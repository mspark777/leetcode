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

type stackNode struct {
	node *TreeNode
	path int
}

func pseudoPalindromicPaths(root *TreeNode) int {
	result := 0
	stack := []stackNode{{node: root, path: 0}}
	for len(stack) > 0 {
		topidx := len(stack) - 1
		top := stack[topidx]
		stack = stack[:topidx]

		node := top.node
		if node == nil {
			continue
		}

		val := node.Val
		left := node.Left
		right := node.Right
		path := top.path ^ (1 << val)

		if (left == nil) && (right == nil) {
			if (path & (path - 1)) == 0 {
				result += 1
			}
		} else {
			stack = append(
				stack,
				stackNode{node: left, path: path},
				stackNode{node: right, path: path},
			)
		}
	}

	return result
}

type input struct {
	root *TreeNode
}

func main() {
	inputs := []input{
		{
			root: newnode(2, newnode(3, newval(3), newval(1)), newright(1, newval(1))),
		},
		{
			root: newnode(2, newnode(1, newval(1), newright(3, newval(1))), newval(1)),
		},
		{
			root: newval(9),
		},
	}

	for _, input := range inputs {
		root := input.root
		result := pseudoPalindromicPaths(root)
		fmt.Println(result)
	}
}
