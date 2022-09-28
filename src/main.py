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
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        if head is None:
            return None

        right = head
        for _ in range(n):
            right = right.next  # type: ignore

        if right is None:
            return head.next

        left = head
        while right.next is not None:
            right = right.next
            left = left.next  # type: ignore

        if left is not None:
            left.next = left.next.next if left.next is not None else None

        return head


class Input:
    nums: list[int]
    n: int

    def __init__(self, nums: list[int], n: int) -> None:
        self.nums = nums
        self.n = n

    def tolist(self) -> Optional[ListNode]:
        dummy = ListNode()
        tail = dummy

        for num in self.nums:
            tail.next = ListNode(num)
            tail = tail.next

        return dummy.next

    def toarr(self, node: Optional[ListNode]) -> list[int]:
        nums: list[int] = []
        while node is not None:
            nums.append(node.val)
            node = node.next

        return nums


def main():
    inputs: list[Input] = [
        Input([1, 2, 3, 4, 5], 2),
        Input([1], 1),
        Input([1, 2], 1),
    ]

    solution = Solution()
    for input in inputs:
        head = input.tolist()
        n = input.n
        result = solution.removeNthFromEnd(head, n)
        print(input.toarr(result))


if __name__ == "__main__":
    main()
