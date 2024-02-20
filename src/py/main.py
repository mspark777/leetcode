from __future__ import annotations
from typing import List


class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        result = 0
        for i, num in enumerate(nums):
            result ^= i ^ num

        return result ^ len(nums)


def main():
    input = ([3, 0, 1], [0, 1], [9, 6, 4, 2, 3, 5, 7, 0, 1])

    for nums in input:
        result = Solution().missingNumber(nums)
        print(result)


if __name__ == "__main__":
    main()
