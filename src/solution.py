"""
solution
"""
from __future__ import annotations
from typing import Optional

class ListNode:
    val: int
    next: Optional[ListNode]
    def __init__(self, x: int):
        self.val = x
        self.next = None

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        fast = head
        slow = head

        while (fast is not None) and (fast.next is not None):
            fast = fast.next.next
            slow = slow.next if slow is not None else None
            if fast is slow:
                return True

        return False
