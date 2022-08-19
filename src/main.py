"""
main
"""

from __future__ import annotations
from typing import Optional
from collections import Counter


class Solution:
    def isPossible(self, nums: list[int]) -> bool:
        lefts = Counter(nums)
        ends = Counter()

        for cur in nums:
            if lefts[cur] == 0:
                continue

            lefts[cur] -= 1

            before1 = cur - 1
            if ends[before1] > 0:
                ends[before1] -= 1
                ends[cur] += 1
                continue

            after1 = cur + 1
            after2 = cur + 2
            if lefts[after1] > 0 and lefts[after2] > 0:
                lefts[after1] -= 1
                lefts[after2] -= 1
                ends[after2] += 1
                continue

            return False
        return True


class Input:
    nums: list[int]

    def __init__(self, nums: list[int]):
        self.nums = nums


def main():
    inputs: list[Input] = [
        Input([1, 2, 3, 3, 4, 5]),
        Input([1, 2, 3, 3, 4, 4, 5, 5]),
        Input([1, 2, 3, 4, 4, 5]),
    ]

    solution = Solution()
    for i in inputs:
        nums = i.nums
        result = solution.isPossible(nums)
        print(result)


if __name__ == "__main__":
    main()
