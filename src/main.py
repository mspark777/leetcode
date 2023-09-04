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

        while (fast is not None) and (fast.next is not None) and (slow is not None):
            fast = fast.next.next
            slow = slow.next

            if fast is slow:
                return True

        return False


def arrtolist(nums: list[int], pos: int) -> Optional[ListNode]:
    dummy = ListNode(0)
    tail = dummy
    cycle: Optional[ListNode] = None
    for i, num in enumerate(nums):
        node = ListNode(num)
        tail.next = node
        tail = node
        if pos == i:
            cycle = node

    if cycle is not None:
        tail.next = cycle

    return dummy.next


def main():
    inputs = [([3, 2, 0, -4], 1), ([1, 2], 0), ([1], -1)]

    for nums, pos in inputs:
        head = arrtolist(nums, pos)
        solution = Solution()
        result = solution.hasCycle(head)
        print(result)


if __name__ == "__main__":
    main()
