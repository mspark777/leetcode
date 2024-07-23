from __future__ import annotations
from collections import Counter
from typing import List


class Solution:
    def frequencySort(self, nums: List[int]) -> List[int]:
        counts = Counter(nums)
        return sorted(nums, key=lambda x: (counts[x], -x))


def main():
    inputs: list[list[int]] = [
        [1, 1, 2, 2, 2, 3],
        [2, 3, 1, 3, 2],
        [-1, 1, -6, 4, 5, -6, 1, 4, 1],
    ]

    for nums in inputs:
        result = Solution().frequencySort(nums)
        print(result)


if __name__ == "__main__":
    main()
