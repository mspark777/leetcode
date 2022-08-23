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
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        nums: list[int] = []
        while head is not None:
            nums.append(head.val)
            head = head.next

        i = 0
        j = len(nums) - 1
        while i < j:
            if nums[i] != nums[j]:
                return False
            else:
                i += 1
                j -= 1
        return True


class Input:
    nums: list[int]

    def __init__(self, nums: list[int]):
        self.nums = nums

    def to_list(self) -> Optional[ListNode]:
        dummy = ListNode()
        tail = dummy
        for num in self.nums:
            tail.next = ListNode(num)
            tail = tail.next
        return dummy.next


def main():
    inputs: list[Input] = [Input([1, 2, 2, 1]), Input([1, 2])]

    solution = Solution()
    for input in inputs:
        result = solution.isPalindrome(input.to_list())
        print(result)


if __name__ == "__main__":
    main()
