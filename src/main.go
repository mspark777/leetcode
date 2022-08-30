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

func createTreeNode(val int, left *TreeNode, right *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Left: left, Right: right}
}

type stackNode struct {
	node *TreeNode
	path []*TreeNode
}

func createNode(node *TreeNode, path []*TreeNode) *stackNode {
	return &stackNode{node: node, path: path}
}

func binaryTreePaths(root *TreeNode) []string {
	if root == nil {
		return []string{}
	}

	stack := []*stackNode{
		createNode(root, []*TreeNode{}),
	}

	result := []string{}
	for len(stack) > 0 {
		topidx := len(stack) - 1
		top := stack[topidx]
		stack = stack[:topidx]

		node := top.node
		left := node.Left
		right := node.Right
		path := append(top.path, node)

		if (left == nil) && (right == nil) {
			spath := make([]string, len(path))
			for i, p := range path {
				spath[i] = fmt.Sprint(p.Val)
			}
			result = append(result, strings.Join(spath, "->"))
			continue
		}

		if left != nil {
			newPath := make([]*TreeNode, len(path))
			copy(newPath, path)
			stack = append(stack, createNode(left, newPath))
		}

		if right != nil {
			newPath := make([]*TreeNode, len(path))
			copy(newPath, path)
			stack = append(stack, createNode(right, newPath))
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
			root: createTreeNode(1, createTreeNode(2, nil, createTreeNode(5, nil, nil)), createTreeNode(3, nil, nil)),
		},
		{
			root: createTreeNode(1, nil, nil),
		},
	}

	for _, input := range inputs {
		root := input.root
		result := binaryTreePaths(root)
		fmt.Println(result)
	}
}
