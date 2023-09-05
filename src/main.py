from __future__ import annotations
from typing import Optional


class ListNode:
    val: int
    next: Optional[ListNode]

    def __init__(self, val: int = 0, next: Optional[ListNode] = None):
        self.val = val
        self.next = next


class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(0, head)
        node = dummy

        while (node.next is not None) and (node.next.next is not None):
            if node.next.val == node.next.next.val:
                while (
                    (node.next is not None)
                    and (node.next.next is not None)
                    and (node.next.val == node.next.next.val)
                ):
                    node.next = node.next.next
                node.next = node.next.next
            else:
                node = node.next

        return dummy.next


def arrtolist(nums: list[int]) -> Optional[ListNode]:
    dummy = ListNode()
    tail = dummy
    for num in nums:
        next = ListNode(num)
        tail.next = next
        tail = next

    return dummy.next


def listtoarr(node: Optional[ListNode]) -> list[int]:
    nums: list[int] = []

    while node is not None:
        nums.append(node.val)
        node = node.next

    return nums


def main():
    inputs = [[1, 2, 3, 3, 4, 4, 5], [1, 1, 1, 2, 3]]

    for nums in inputs:
        solution = Solution()
        result = solution.deleteDuplicates(arrtolist(nums))
        print(listtoarr(result))


if __name__ == "__main__":
    main()
