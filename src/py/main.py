from __future__ import annotations
from typing import List
from functools import reduce


class Solution:
    def singleNumber(self, nums: List[int]) -> List[int]:
        diff = reduce(lambda acc, cur: acc ^ cur, nums, 0)
        diff &= -diff

        left = 0
        right = 0
        for num in nums:
            if num & diff == 0:
                left ^= num
            else:
                right ^= num

        return [left, right]


def main():
    inputs = ([1, 2, 1, 3, 2, 5], [-1, 0], [0, 1])

    for nums in inputs:
        result = Solution().singleNumber(nums)
        print(result)


if __name__ == "__main__":
    main()
