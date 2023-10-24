from __future__ import annotations
from typing import Optional, List


class ListNode:
    val: int
    next: Optional[ListNode]

    def __init__(self, val: int = 0, next: Optional[ListNode] = None):
        self.val = val
        self.next = next


class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        """
        Do not return anything, modify head in-place instead.
        """
        if head is None:
            return
        elif head.next is None:
            return

        slow: Optional[ListNode] = head
        fast: Optional[ListNode] = head
        while fast is not None and fast.next is not None and slow is not None:
            slow = slow.next
            fast = fast.next.next

        new_node: Optional[ListNode] = None
        if slow is not None:
            new_node = self.reverse(slow.next)
            slow.next = None

        curr: Optional[ListNode] = head
        dummy = new_node

        while curr is not None and dummy is not None:
            temp = curr.next
            curr.next = dummy
            temp2 = dummy.next

            dummy.next = temp
            curr = temp
            dummy = temp2

    def reverse(self, node: Optional[ListNode]) -> Optional[ListNode]:
        prev: Optional[ListNode] = None
        curr: Optional[ListNode] = node
        next: Optional[ListNode] = None

        while curr is not None:
            next = curr.next
            curr.next = prev
            prev = curr
            curr = next

        return prev


def atol(vals: list[int]) -> Optional[ListNode]:
    dummy = ListNode()
    tail = dummy

    for v in vals:
        next = ListNode(v)
        tail.next = next
        tail = next

    return dummy.next


def ltoa(node: Optional[ListNode]) -> list[int]:
    vals: list[int] = []
    while node is not None:
        vals.append(node.val)
        node = node.next

    return vals


def main():
    inputs = ([1, 2, 3, 4], [1, 2, 3, 4, 5])

    for vals in inputs:
        head = atol(vals)
        Solution().reorderList(head)
        print(ltoa(head))


if __name__ == "__main__":
    main()
