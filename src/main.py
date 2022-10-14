from __future__ import annotations
from typing import Optional, List


class ListNode:
    val: int
    next: Optional[ListNode]

    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def listarr(node: Optional[ListNode]) -> list[int]:
    nums: list[int] = []

    while node is not None:
        nums.append(node.val)
        node = node.next

    return nums


def arrlist(nums: list[int]) -> Optional[ListNode]:
    temp = ListNode()
    tail = temp

    for num in nums:
        tail.next = ListNode(num)
        tail = tail.next

    return temp.next


class Solution:
    def deleteMiddle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head:
            return None

        if not head.next:
            return None

        slow = head
        fast = head.next.next

        while slow and fast and fast.next:
            slow = slow.next
            fast = fast.next.next

        if slow and slow.next:
            slow.next = slow.next.next

        return head


def main():
    inputs: list[list[int]] = [[1, 3, 4, 7, 1, 2, 6], [1, 2, 3, 4], [2, 1]]

    solution = Solution()
    for nums in inputs:
        result = solution.deleteMiddle(arrlist(nums))
        print(listarr(result))


if __name__ == "__main__":
    main()
