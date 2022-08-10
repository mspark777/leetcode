package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func travel(nums []int, l int, r int) *TreeNode {
	if l >= r {
		return nil
	}

	mid := (l + r) / 2
	return &TreeNode{
		nums[mid],
		travel(nums, l, mid),
		travel(nums, mid+1, r),
	}
}

func sortedArrayToBST(nums []int) *TreeNode {
	return travel(nums, 0, len(nums))
}

type input struct {
	nums []int
}

func main() {
	inputs := []*input{
		{
			nums: []int{-10, -3, 0, 5, 9},
		},
		{
			nums: []int{1, 3},
		},
	}

	for _, input := range inputs {
		nums := input.nums
		result := sortedArrayToBST(nums)
		fmt.Println(result)
	}
}
