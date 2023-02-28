package main

import (
	"fmt"
)

func traverse(node *TreeNode, triplets map[string]int, counts map[int]int, result *[]*TreeNode) int {
	if node == nil {
		return 0
	}

	triplet := fmt.Sprint(
		traverse(node.Left, triplets, counts, result),
		node.Val,
		traverse(node.Right, triplets, counts, result),
	)

	if _, ok := triplets[triplet]; !ok {
		triplets[triplet] = len(triplets) + 1
	}

	id := triplets[triplet]
	counts[id] += 1
	if counts[id] == 2 {
		*result = append(*result, node)
	}

	return id
}

func findDuplicateSubtrees(root *TreeNode) []*TreeNode {
	result := []*TreeNode{}
	traverse(root, map[string]int{}, map[int]int{}, &result)
	return result
}

func main() {
	inputs := []*TreeNode{
		newTreeNode(1,
			newTreeLeft(2, newTreeVal(4)),
			newTreeNode(3, newTreeLeft(2, newTreeVal(4)), newTreeVal(4)),
		),
		newTreeNode(2, newTreeVal(1), newTreeVal(1)),
		newTreeNode(2, newTreeLeft(2, newTreeVal(3)), newTreeLeft(2, newTreeVal(3))),
	}

	for _, root := range inputs {
		result := findDuplicateSubtrees(root)
		fmt.Println(result)
	}
}
