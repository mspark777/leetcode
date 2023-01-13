from __future__ import annotations
from typing import Counter, List
from collections import Counter


class Solution:
    def intersect(self, nums1: List[int], nums2: List[int]) -> List[int]:
        counts = Counter(nums1)

        result: list[int] = []
        for num in nums2:
            if num in counts:
                count = counts[num]
                if count > 0:
                    result.append(num)
                    counts[num] = count - 1

        return result


def main():
    inputs: list[list[list[int]]] = [
        [[1, 2, 2, 1], [2, 2]],
        [[4, 9, 5], [9, 4, 9, 8, 4]],
    ]

    for [nums1, nums2] in inputs:
        solution = Solution()
        result = solution.intersect(nums1, nums2)
        print(result)


if __name__ == "__main__":
    main()
