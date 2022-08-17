"""
main
"""

from __future__ import annotations
from typing import Optional


class ListNode:
    val: int
    next: Optional[ListNode]

    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def removeElements(self, head: Optional[ListNode], val: int) -> Optional[ListNode]:
        dummy = ListNode(-1, head)
        cur = dummy

        while cur is not None:
            while (cur.next is not None) and (cur.next.val == val):
                cur.next = cur.next.next
            cur = cur.next
        return dummy.next


class Input:
    nums: list[int]
    val: int

    def __init__(self, nums: list[int], val: int):
        self.nums = nums
        self.val = val

    def to_list(self) -> Optional[ListNode]:
        head = ListNode()
        tail = head

        for num in self.nums:
            tail.next = ListNode(num)
            tail = tail.next

        return head.next

    def to_arr(self, head: Optional[ListNode]) -> list[int]:
        nums: list[int] = []
        while head is not None:
            nums.append(head.val)
            head = head.next

        return nums


def main():
    inputs: list[Input] = [
        Input([1, 2, 6, 3, 4, 5, 6], 6),
        Input([], 1),
        Input([7, 7, 7, 7], 7),
    ]

    solution = Solution()
    for i in inputs:
        result = solution.removeElements(i.to_list(), i.val)
        print(i.to_arr(result))


if __name__ == "__main__":
    main()
