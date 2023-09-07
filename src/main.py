from __future__ import annotations
from typing import Optional


class ListNode:
    val: int
    next: Optional[ListNode]

    def __init__(self, val: int = 0, next: Optional[ListNode] = None):
        self.val = val
        self.next = next


class Solution:
    def reverseBetween(
        self, head: Optional[ListNode], left: int, right: int
    ) -> Optional[ListNode]:
        if head is None:
            return None

        cur = head
        prev: Optional[ListNode] = None
        while (left > 1) and (cur is not None):
            prev = cur
            cur = cur.next
            left -= 1
            right -= 1

        tail = cur
        con = prev
        while (right > 0) and (cur is not None):
            third = cur.next
            cur.next = prev
            prev = cur
            cur = third
            right -= 1

        if con is not None:
            con.next = prev
        else:
            head = prev

        if tail is not None:
            tail.next = cur

        return head


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
    inputs = [([1, 2, 3, 4, 5], 2, 4), ([5], 1, 1)]

    for nums, left, right in inputs:
        solution = Solution()
        result = solution.reverseBetween(arrtolist(nums), left, right)
        print(listtoarr(result))


if __name__ == "__main__":
    main()
