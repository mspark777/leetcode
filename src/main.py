"""
main
"""

from __future__ import annotations
from typing import Optional

class TreeNode:
    val: int
    left: Optional[TreeNode]
    right: Optional[TreeNode]
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def sortedArrayToBST(self, nums: list[int]) -> Optional[TreeNode]:
        return self.travel(nums, 0, len(nums))

    def travel(self, nums: list[int], l: int, r: int) -> Optional[TreeNode]:
        if l >= r:
            return None

        mid = (l + r) // 2
        return TreeNode(
                nums[mid],
                self.travel(nums, l, mid),
                self.travel(nums, mid + 1, r)
        )

class Input:
    nums: list[int]
    def __init__(self, nums: list[int]):
        self.nums = nums

def main():
    inputs: list[Input] = [
            Input([-10, -3, 0, 5, 9]),
            Input([1, 3]),
    ]

    s = Solution()
    for i in inputs:
        result = s.sortedArrayToBST(i.nums)
        print(result)

if __name__ == "__main__":
    main()
