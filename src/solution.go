package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func reverse(nums []int) {
	l := len(nums)
	mid := l / 2
	for i := 0; i < mid; i += 1 {
		j := l - (i + 1)
		nums[i], nums[j] = nums[j], nums[i]
	}
}

func postorderTraversal(root *TreeNode) []int {
	result := []int{}
	if root == nil {
		return result
	}

	stack := []*TreeNode{root}
	for l := len(stack); l > 0; l = len(stack) {
		top := l - 1
		node := stack[top]
		stack = stack[:top]

		result = append(result, node.Val)
		left := node.Left
		if left != nil {
			stack = append(stack, left)
		}

		right := node.Right
		if right != nil {
			stack = append(stack, right)
		}
	}

	reverse(result)
	return result
}
