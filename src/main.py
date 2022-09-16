"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def maximumScore(self, nums: list[int], multipliers: list[int]) -> int:
        n = len(nums)
        m = len(multipliers)
        dp = [0 for _ in range(m + 1)]

        for op in range(m - 1, -1, -1):
            row = dp.copy()

            for left in range(op, -1, -1):
                n0 = multipliers[op] * nums[left] + row[left + 1]
                n1 = multipliers[op] * nums[n - 1 - (op - left)] + row[left]
                dp[left] = max(n0, n1)

        return dp[0]


class Input:
    nums: list[int]
    multipliers: list[int]

    def __init__(self, nums: list[int], multipliers: list[int]):
        self.nums = nums
        self.multipliers = multipliers


def main():
    inputs: list[Input] = [
        Input([1, 2, 3], [3, 2, 1]),
        Input([-5, -3, -3, -2, 7, 1], [-10, -5, 3, 4, 6]),
    ]

    solution = Solution()
    for input in inputs:
        nums = input.nums
        multipliers = input.multipliers
        result = solution.maximumScore(nums, multipliers)
        print(result)


if __name__ == "__main__":
    main()
