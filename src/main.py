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
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        return self.reverse_recursively(head)

    def reverse_iteratively(self, node: Optional[ListNode]) -> Optional[ListNode]:
        prev: Optional[ListNode] = None

        while node is not None:
            next = node.next
            node.next = prev
            prev = node
            node = next
        return prev

    def reverse_recursively(self, node: Optional[ListNode]) -> Optional[ListNode]:
        if node is None:
            return node

        next = node.next
        if next is None:
            return node

        new_node = self.reverse_recursively(next)
        next.next = node
        node.next = None
        return new_node


class Input:
    nums: list[int]

    def __init__(self, nums: list[int]):
        self.nums = nums

    def tolist(self) -> Optional[ListNode]:
        dummy = ListNode()
        tail = dummy
        for num in self.nums:
            tail.next = ListNode(num)
            tail = tail.next
        return dummy.next

    def fromlist(self, node: Optional[ListNode]) -> list[int]:
        nums: list[int] = []
        while node is not None:
            nums.append(node.val)
            node = node.next
        return nums


def main():
    inputs: list[Input] = [Input([1, 2, 3, 4, 5]), Input([1, 2]), Input([])]

    solution = Solution()
    for i in inputs:
        result = solution.reverseList(i.tolist())
        print(i.fromlist(result))


if __name__ == "__main__":
    main()
