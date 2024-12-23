package main

import (
	"fmt"
	"sort"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func minimumOperations(root *TreeNode) int {
	var SHIFT int = 20
	var MASK int = 0xFFFFF

	queue := []*TreeNode{root}
	result := 0

	for len(queue) > 0 {
		levelSize := len(queue)
		nodes := make([]int, levelSize)

		for i := 0; i < levelSize; i += 1 {
			node := queue[i]
			nodes[i] = (node.Val << SHIFT) + i

			if node.Left != nil {
				queue = append(queue, node.Left)
			}

			if node.Right != nil {
				queue = append(queue, node.Right)
			}
		}

		sort.Ints(nodes)

		for i := 0; i < levelSize; i += 1 {
			origPos := nodes[i] & MASK
			if origPos != i {
				left := nodes[i]
				right := nodes[origPos]
				nodes[i] = right
				nodes[origPos] = left
				result += 1
				i -= 1
			}
		}

		queue = queue[levelSize:]
	}

	return result
}

type input struct {
	names   []string
	heights []int
}

func main() {
	inputs := []input{
		{
			[]string{"Mary", "John", "Emma"},
			[]int{180, 165, 170},
		},
		{
			[]string{"Alice", "Bob", "Bob"},
			[]int{155, 185, 150},
		},
		{
			[]string{"IEO", "Sgizfdfrims", "QTASHKQ", "Vk", "RPJOFYZUBFSIYp", "EPCFFt", "VOYGWWNCf", "WSpmqvb"},
			[]int{17233, 32521, 14087, 42738, 46669, 65662, 43204, 8224},
		},
	}

	for _, input := range inputs {
		result := sortPeople(input.names, input.heights)
		fmt.Println(result)
	}
}
