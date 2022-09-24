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

func newval(val int) *TreeNode {
	return newnode(val, nil, nil)
}

type stackNode struct {
	path []int
	node *TreeNode
	sum  int
}

func pathSum(root *TreeNode, targetSum int) [][]int {
	result := [][]int{}
	if root == nil {
		return result
	}

	stack := []stackNode{{
		path: []int{},
		node: root,
		sum:  0,
	}}

	for len(stack) > 0 {
		topidx := len(stack) - 1
		top := stack[topidx]
		stack = stack[:topidx]

		path := top.path
		sum := top.sum
		node := top.node
		val := node.Val
		left := node.Left
		right := node.Right
		newsum := sum + val
		isLeaf := true

		if left != nil {
			isLeaf = false
			newpath := make([]int, len(path)+1)
			copy(newpath, path)
			newpath[len(path)] = val
			stack = append(stack, stackNode{
				path: newpath,
				node: left,
				sum:  newsum,
			})
		}

		if right != nil {
			isLeaf = false
			newpath := make([]int, len(path)+1)
			copy(newpath, path)
			newpath[len(path)] = val
			stack = append(stack, stackNode{
				path: newpath,
				node: right,
				sum:  newsum,
			})
		}

		if isLeaf && (newsum == targetSum) {
			newpath := make([]int, len(path)+1)
			copy(newpath, path)
			newpath[len(path)] = val
			result = append(result, newpath)
		}
	}
	return result
}

type input struct {
	root      *TreeNode
	targetSum int
}

func main() {
	inputs := []input{
		{
			root:      newnode(5, newleft(4, newnode(11, newval(7), newval(2))), newnode(8, newval(13), newnode(4, newval(5), newval(1)))),
			targetSum: 22,
		},
		{
			root:      newnode(1, newval(2), newval(3)),
			targetSum: 5,
		},
		{
			root:      newleft(1, newval(2)),
			targetSum: 0,
		},
	}

	for _, input := range inputs {
		root := input.root
		targetSum := input.targetSum
		result := pathSum(root, targetSum)
		fmt.Println(result)
	}
}
