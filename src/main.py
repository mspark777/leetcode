"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def containsNearbyDuplicate(self, nums: list[int], k: int) -> bool:
        index_map: dict[int, int] = {}
        for i, num in enumerate(nums):
            if num in index_map and ((i - index_map[num]) <= k):
                return True
            index_map[num] = i
        return False


class Input:
    nums: list[int]
    k: int

    def __init__(self, nums: list[int], k: int):
        self.nums = nums
        self.k = k


def main():
    inputs: list[Input] = [
        Input([1, 2, 3, 1], 3),
        Input([1, 0, 1, 1], 1),
        Input([1, 2, 3, 1, 2, 3], 2),
    ]

    solution = Solution()
    for i in inputs:
        result = solution.containsNearbyDuplicate(i.nums, i.k)
        print(result)


if __name__ == "__main__":
    main()
