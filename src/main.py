from __future__ import annotations
from collections import Counter
from typing import List, Optional


class ListNode:
    val: int
    next: Optional[ListNode]

    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        slow = head
        fast = head

        while (fast is not None) and (fast.next is not None):
            slow = slow.next if slow is not None else None
            fast = fast.next.next

        return slow


def arrtolist(nums: list[int]) -> Optional[ListNode]:
    head = ListNode()
    tail = head

    for val in nums:
        tail.next = ListNode(val)
        tail = tail.next

    return head.next


def listtoarr(node: Optional[ListNode]) -> list[int]:
    nums: list[int] = []

    while node:
        nums.append(node.val)
        node = node.next

    return nums


def main():
    inputs: list[list[int]] = [[1, 2, 3, 4, 5], [1, 2, 3, 4, 5, 6]]

    solution = Solution()
    for nums in inputs:
        result = solution.middleNode(arrtolist(nums))
        print(listtoarr(result))


if __name__ == "__main__":
    main()
