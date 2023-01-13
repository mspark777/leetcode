from __future__ import annotations
from typing import List


class Solution:
    def intersection(self, nums1: List[int], nums2: List[int]) -> List[int]:
        filter1 = set(nums1)
        filter2 = set(nums2)

        return [i for i in filter1 if i in filter2]


def main():
    inputs: list[list[list[int]]] = [
        [[1, 2, 2, 1], [2, 2]],
        [[4, 9, 5], [9, 4, 9, 8, 4]],
    ]

    for [nums1, nums2] in inputs:
        solution = Solution()
        result = solution.intersection(nums1, nums2)
        print(result)


if __name__ == "__main__":
    main()
