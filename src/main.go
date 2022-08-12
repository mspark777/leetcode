package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	if (p == nil) || (q == nil) {
		return nil
	}

	pval := p.Val
	qval := q.Val
	cur := root
	for cur != nil {
		val := cur.Val
		if (pval < val) && (qval < val) {
			cur = cur.Left
		} else if (pval > val) && (qval > val) {
			cur = cur.Right
		} else {
			break
		}
	}

	return cur
}

type input struct {
	root *TreeNode
	qval int
	pval int
}

func main() {
	inputs := []*input{}

	for _, input := range inputs {
		root := input.root
		result := lowestCommonAncestor(root, nil, nil)
		fmt.Println(result)
	}
}
