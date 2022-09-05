package main

import "fmt"

type Node struct {
	Val      int
	Children []*Node
}

func levelOrder(root *Node) [][]int {
	result := [][]int{}
	if root == nil {
		return result
	}

	queue := []*Node{root}
	for len(queue) > 0 {
		size := len(queue)
		values := make([]int, size)
		for i := 0; i < size; i += 1 {
			node := queue[i]
			values[i] = node.Val
			queue = append(queue, node.Children...)
		}
		queue = queue[size:]
		result = append(result, values)
	}

	return result
}

type input struct {
	root *Node
}

func newnode(val int, children ...*Node) *Node {
	return &Node{Val: val, Children: children}
}

func main() {
	inputs := []input{
		{
			root: newnode(1,
				newnode(3,
					newnode(5),
					newnode(6),
				),
				newnode(2),
				newnode(4),
			),
		},
		{
			root: newnode(1,
				newnode(2),
				newnode(3,
					newnode(6),
					newnode(7,
						newnode(11,
							newnode(14),
						),
					),
				),
				newnode(4,
					newnode(8,
						newnode(12),
					),
				),
				newnode(5,
					newnode(9,
						newnode(13),
					),
					newnode(10),
				),
			),
		},
	}

	for _, input := range inputs {
		result := levelOrder(input.root)
		fmt.Println(result)
	}
}
