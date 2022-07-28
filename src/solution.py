"""
solution
"""
from __future__ import annotations
from typing import  Optional

class ListNode:
    val: int
    x: Optional[ListNode]
    def __init__(self, x: int, next: Optional[ListNode]):
        self.val = x
        self.next = next

class Solution:
    def getIntersectionNode(self, headA: Optional[ListNode], headB: Optional[ListNode]) -> Optional[ListNode]:
        a = headA
        b = headB
        while a is not b:
            a = a.next if a is not None else headB
            b = b.next if b is not None else headA
        return a
