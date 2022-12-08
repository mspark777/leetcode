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

func dfs(stack []int, node *TreeNode) []int {
	if node == nil {
		return stack
	}

	val := node.Val
	left := node.Left
	right := node.Right

	if (left == nil) && (right == nil) {
		stack = append(stack, val)
	}

	stack = dfs(stack, left)
	return dfs(stack, right)
}

func leafSimilar(root1 *TreeNode, root2 *TreeNode) bool {
	stack1 := dfs([]int{}, root1)
	stack2 := dfs([]int{}, root2)

	if len(stack1) != len(stack2) {
		return false
	}

	for i, v1 := range stack1 {
		if v1 != stack2[i] {
			return false
		}
	}

	return true
}

type input struct {
	root1 *TreeNode
	root2 *TreeNode
}

func main() {
	inputs := []input{
		{
			root1: newnode(3,
				newnode(5,
					newval(6),
					newnode(2,
						newval(7),
						newval(4),
					),
				),
				newnode(1,
					newval(9),
					newval(8),
				),
			),
			root2: newnode(3,
				newnode(5, newval(6), newval(7)),
				newnode(1,
					newval(4),
					newnode(2,
						newval(9),
						newval(8),
					),
				),
			),
		},
		{
			root1: newnode(1, newval(2), newval(3)),
			root2: newnode(1, newval(3), newval(2)),
		},
	}

	for _, input := range inputs {
		root1 := input.root1
		root2 := input.root2
		result := leafSimilar(root1, root2)
		fmt.Println(result)
	}
}
