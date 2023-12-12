from __future__ import annotations
from typing import List


class Solution:
    def findMin(self, nums: List[int]) -> int:
        n = len(nums)
        left = 0
        right = n - 1
        result = 10000

        while left <= right:
            mid = (left + right) // 2
            m = nums[mid]
            l = nums[left]
            r = nums[right]

            result = min(result, m)
            if l < r and l < m:
                right = mid - 1
            else:
                left = left + 1

        return result


def main():
    inputs = ([3, 4, 5, 1, 2], [4, 5, 6, 7, 0, 1, 2], [11, 13, 15, 17])

    for nums in inputs:
        result = Solution().findMin(nums)
        print(result)


if __name__ == "__main__":
    main()
