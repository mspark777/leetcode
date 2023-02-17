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

type data struct {
	prev        *TreeNode
	minDistance int
}

func travel(root *TreeNode, data *data) {
	if root == nil {
		return
	}

	travel(root.Left, data)

	if data.prev != nil {
		distance := root.Val - data.prev.Val
		if distance < data.minDistance {
			data.minDistance = distance
		}
	}
	data.prev = root
	travel(root.Right, data)
}

func minDiffInBST(root *TreeNode) int {
	data := &data{minDistance: 2147483647}
	travel(root, data)

	return data.minDistance
}

func main() {
	inputs := []*TreeNode{
		newnode(4, newnode(2, newval(1), newval(3)), newval(6)),
		newnode(1, newval(0), newnode(48, newval(12), newval(49))),
		newright(27, newright(34, newleft(58, newleft(50, newval(44))))),
	}

	for _, input := range inputs {
		result := minDiffInBST(input)
		fmt.Println(result)
	}
}
