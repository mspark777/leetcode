"""
solution
"""
from __future__ import annotations
from typing import Optional

class ListNode:
    val: int
    next: Optional[ListNode]  # type: ignore

    def __init__(self, val=0, n=None):
        self.val = val
        self.next = n

class Solution:
    def partition(self, head: Optional[ListNode], x: int) -> Optional[ListNode]:
        before_head = ListNode()
        after_head = ListNode()
        before = before_head
        after = after_head

        while head is not None:
            if head.val < x:
                before.next = head
                before = before.next
            else:
                after.next = head
                after = after.next

            head = head.next

        after.next = None
        before.next = after_head.next

        return before_head.next
