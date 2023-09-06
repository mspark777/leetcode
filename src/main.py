from __future__ import annotations
from typing import Optional, List


class ListNode:
    val: int
    next: Optional[ListNode]

    def __init__(self, val: int = 0, next: Optional[ListNode] = None):
        self.val = val
        self.next = next


class Solution:
    def splitListToParts(
        self, head: Optional[ListNode], k: int
    ) -> List[Optional[ListNode]]:
        cur = head
        node_count = 0
        while cur is not None:
            cur = cur.next
            node_count += 1

        width, rem = divmod(node_count, k)

        cur = head
        result: list[Optional[ListNode]] = []
        for i in range(k):
            root = cur
            for _ in range(width + (0 if i < rem else -1)):
                if cur is not None:
                    cur = cur.next

            if cur is not None:
                prev = cur
                cur = cur.next
                prev.next = None

            result.append(root)

        return result


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
    inputs = [([1, 2, 3], 5), ([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3)]

    for nums, k in inputs:
        solution = Solution()
        result = solution.splitListToParts(arrtolist(nums), k)
        print([listtoarr(n) for n in result])


if __name__ == "__main__":
    main()
