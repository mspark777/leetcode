"""
main
"""

from __future__ import annotations
from typing import Optional
from collections import Counter


class Solution:
    def moveZeroes(self, nums: list[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        last_zero = 0

        for i in range(len(nums)):
            if nums[i] != 0:
                nums[last_zero] = nums[i]
                last_zero += 1

        for i in range(last_zero, len(nums)):
            nums[i] = 0


class Input:
    nums: list[int]

    def __init__(self, nums: list[int]):
        self.nums = nums


def main():
    inputs: list[Input] = [
        Input([0, 1, 0, 3, 12]),
        Input([0]),
    ]

    solution = Solution()
    for i in inputs:
        nums = i.nums
        solution.moveZeroes(nums)
        print(nums)


if __name__ == "__main__":
    main()
