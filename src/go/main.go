package main

import (
	"fmt"
)

func detectCycle(head *ListNode) *ListNode {
	fast := head
	slow := head

	for (fast != nil) && (fast.Next != nil) {
		fast = fast.Next.Next
		slow = slow.Next

		if fast == slow {
			break
		}
	}

	if (fast == nil) || (fast.Next == nil) {
		return nil
	}

	fast = head
	for fast != slow {
		fast = fast.Next
		slow = slow.Next
	}

	return fast
}

func main() {
	inputs := []*ListNode{
		newCycleList([]int{3, 2, 0, -4}, 1),
		newCycleList([]int{1, 2}, 0),
		newCycleList([]int{1}, -1),
	}

	for _, head := range inputs {
		result := detectCycle(head)
		if result != nil {
			fmt.Println(result.Val)
		} else {
			fmt.Println(nil)
		}
	}
}
