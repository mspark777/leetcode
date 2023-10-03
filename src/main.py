from __future__ import annotations
from typing import Optional, List
from collections import defaultdict


class Solution:
    def numIdenticalPairs(self, nums: List[int]) -> int:
        counts = defaultdict[int, int](int)
        result = 0
        for num in nums:
            cnt = counts[num]
            result += cnt
            counts[num] += 1

        return result


def main():
    inputs = [[1, 2, 3, 1, 1, 3], [1, 1, 1, 1], [1, 2, 3]]

    for nums in inputs:
        solution = Solution()
        result = solution.numIdenticalPairs(nums)
        print(result)


if __name__ == "__main__":
    main()
