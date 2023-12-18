from __future__ import annotations
from typing import List


class Solution:
    def maxProductDifference(self, nums: List[int]) -> int:
        w = 0
        x = 0
        y = 10**5
        z = 10**5

        for num in nums:
            if num > w:
                x = w
                w = num
            elif num > x:
                x = num

            if num < y:
                z = y
                y = num
            elif num < z:
                z = num

        return (w * x) - (y * z)


def main():
    inputs = ([5, 6, 2, 7, 4], [4, 2, 5, 9, 7, 4, 8])

    for nums in inputs:
        result = Solution().maxProductDifference(nums)
        print(result)


if __name__ == "__main__":
    main()
