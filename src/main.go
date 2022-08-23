package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func isPalindrome(head *ListNode) bool {
	nums := []int{}
	for head != nil {
		nums = append(nums, head.Val)
		head = head.Next
	}

	for i, j := 0, len(nums)-1; i < j; i, j = i+1, j-1 {
		if nums[i] != nums[j] {
			return false
		}
	}

	return true
}

type input struct {
	nums []int
}

func arrToList(nums []int) *ListNode {
	dummy := &ListNode{}
	tail := dummy

	for _, num := range nums {
		tail.Next = &ListNode{Val: num}
		tail = tail.Next
	}

	return dummy.Next
}

func main() {
	inputs := []input{
		{
			nums: []int{1, 2, 2, 1},
		},
		{
			nums: []int{1, 2},
		},
	}

	for _, input := range inputs {
		nums := input.nums
		result := isPalindrome(arrToList(nums))
		fmt.Println(result)
	}
}
