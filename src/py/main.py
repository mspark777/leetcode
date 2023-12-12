from __future__ import annotations
from typing import List


class Solution:
    def findPeakElement(self, nums: List[int]) -> int:
        left = 0
        right = len(nums) - 1

        while left < right:
            mid = (left + right) // 2
            if nums[mid] > nums[mid + 1]:
                right = mid
            else:
                left = mid + 1

        return left


def main():
    inputs = ([1, 2, 3, 1], [1, 2, 1, 3, 5, 6, 4])

    for nums in inputs:
        result = Solution().findPeakElement(nums)
        print(result)


if __name__ == "__main__":
    main()
