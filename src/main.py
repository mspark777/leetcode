"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def majorityElement(self, nums: list[int]) -> int:
        count = 0
        candidate = nums[0]

        for num in nums:
            if count < 1:
                candidate = num
            count += 1 if num == candidate else -1
        return candidate


class Input:
    nums: list[int]

    def __init__(self, nums: list[int]):
        self.nums = nums


def main():
    inputs: list[Input] = [
        Input([3, 2, 3]),
        Input([2, 2, 1, 1, 1, 2, 2]),
    ]

    solution = Solution()
    for i in inputs:
        result = solution.majorityElement(i.nums)
        print(result)


if __name__ == "__main__":
    main()
