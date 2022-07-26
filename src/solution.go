package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type temp struct {
	result []int
}

func preorder(root *TreeNode, temp *temp) {
	if root != nil {
		temp.result = append(temp.result, root.Val)
		preorder(root.Left, temp)
		preorder(root.Right, temp)
	}
}

func preorderTraversal(root *TreeNode) []int {
	temp := &temp{result: []int{}}
	preorder(root, temp)

	return temp.result
}
