from __future__ import annotations
from typing import  Optional


class ListNode:
    val: int
    next: Optional[ListNode]

    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return None

        odd = head
        even = head.next
        even_head = even
        while (even is not None) and (even.next is not None):
            odd.next = even.next
            odd = odd.next

            even.next = odd.next
            even = even.next

        odd.next = even_head
        return head


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
    inputs: list[list[int]] = [
            [1, 2, 3, 4, 5], 
            [2, 1, 3, 5, 6, 4, 7]
            ]

    solution = Solution()
    for nums in inputs:
        result = solution.oddEvenList(arrtolist(nums))
        print(listtoarr(result))


if __name__ == "__main__":
    main()

