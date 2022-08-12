"""
main
"""

from __future__ import annotations
from typing import Optional

class Solution:
    def nextPermutation(self, nums: list[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        nlen = len(nums)
        i = nlen - 2
        while (i >= 0) and (nums[i] >= nums[i + 1]):
            i -= 1

        if i >= 0:
            j = nlen - 1
            while nums[i] >= nums[j]:
                j -= 1
            self.swap(nums, i, j)
        self.reverse(nums, i + 1)


    def swap(self, nums: list[int], i: int, j: int) -> None:
        temp = nums[i]
        nums[i] = nums[j]
        nums[j] = temp

    def reverse(self, nums: list[int], start: int) -> None:
        i = start
        j = len(nums) - 1
        while i < j:
            self.swap(nums, i, j)
            i += 1
            j -= 1


class Input:
    nums: list[int]

    def __init__(self, nums: list[int]):
        self.nums = nums

def main():
    inputs: list[Input] = [
            Input([1, 2, 3]),
            Input([3, 2, 1]),
            Input([1, 1, 5]),
    ]

    s = Solution()
    for i in inputs:
        s.nextPermutation(i.nums)
        print(i.nums)

if __name__ == "__main__":
    main()
