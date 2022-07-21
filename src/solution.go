package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseBetween(head *ListNode, left int, right int) *ListNode {
	if head == nil {
		return head
	}

	cur := head
	var prev *ListNode = nil
	for left > 1 {
		prev = cur
		cur = cur.Next
		left -= 1
		right -= 1
	}

	tail := cur
	con := prev
	var third *ListNode = nil
	for right > 0 {
		third = cur.Next
		cur.Next = prev
		prev = cur
		cur = third
		right -= 1
	}

	if con != nil {
		con.Next = prev
	} else {
		head = prev
	}

	tail.Next = cur
	return head
}
