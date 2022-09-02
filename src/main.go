package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func averageOfLevels(root *TreeNode) []float64 {
	result := []float64{}
	if root == nil {
		return result
	}

	queue := []*TreeNode{root}
	for len(queue) > 0 {
		size := len(queue)
		total := 0.0

		for i := 0; i < size; i += 1 {
			head := queue[i]
			total += float64(head.Val)

			left := head.Left
			if left != nil {
				queue = append(queue, left)
			}
			right := head.Right
			if right != nil {
				queue = append(queue, right)
			}
		}
		queue = queue[size:]
		result = append(result, total/float64(size))
	}

	return result
}

type input struct {
	root *TreeNode
}

func newnode(val int, left, right *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Left: left, Right: right}
}

func newval(val int) *TreeNode {
	return newnode(val, nil, nil)
}

func main() {
	inputs := []input{
		{
			root: newnode(3,
				newval(9),
				newnode(20, newval(15), newval(7)),
			),
		},
		{
			root: newnode(3,
				newnode(9, newval(15), newval(7)),
				newval(20),
			),
		},
	}

	for _, input := range inputs {
		root := input.root
		result := averageOfLevels(root)
		fmt.Println(result)
	}
}
