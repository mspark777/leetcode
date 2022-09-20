"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def findLength(self, nums1: list[int], nums2: list[int]) -> int:
        result = 0
        lengths = [[0] * (len(nums2) + 1) for _ in range(len(nums1) + 1)]

        for i in range(len(nums1) - 1, -1, -1):
            for j in range(len(nums2) - 1, -1, -1):
                if nums1[i] != nums2[j]:
                    continue

                length = lengths[i + 1][j + 1] + 1
                lengths[i][j] = length
                result = max(result, length)

        return result


class Input:
    nums1: list[int]
    nums2: list[int]

    def __init__(self, nums1: list[int], nums2: list[int]):
        self.nums1 = nums1
        self.nums2 = nums2


def main():
    inputs: list[Input] = [
        Input([1, 2, 3, 2, 1], [3, 2, 1, 4, 7]),
        Input([0, 0, 0, 0, 0], [0, 0, 0, 0, 0]),
    ]

    solution = Solution()
    for input in inputs:
        nums1 = input.nums1
        nums2 = input.nums2
        result = solution.findLength(nums1, nums2)
        print(result)


if __name__ == "__main__":
    main()
