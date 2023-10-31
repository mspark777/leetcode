from __future__ import annotations
from typing import Optional, List


class ListNode:
    val: int
    next: Optional[ListNode]

    def __init__(self, val: int = 0, next: Optional[ListNode] = None):
        self.val = val
        self.next = next


class Solution:
    def insertionSortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode()
        curr = head

        while curr is not None:
            prev = dummy
            while prev.next and prev.next.val <= curr.val:
                prev = prev.next

            next = curr.next
            curr.next = prev.next
            prev.next = curr
            curr = next

        return dummy.next


def atol(nums: list[int]) -> Optional[ListNode]:
    dummy = ListNode()
    tail = dummy
    for num in nums:
        next = ListNode(num)
        tail.next = next
        tail = next

    return dummy.next


def ltoa(node: Optional[ListNode]) -> list[int]:
    nums: list[int] = []

    while node is not None:
        nums.append(node.val)
        node = node.next

    return nums


def main():
    inputs = ([4, 2, 1, 3], [-1, 5, 3, 4, 0])

    for nums in inputs:
        result = Solution().insertionSortList(atol(nums))
        print(ltoa(result))


if __name__ == "__main__":
    main()
