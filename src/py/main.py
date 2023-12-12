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

            result = min(result, l)
            if m > r:
                left = mid + 1
            elif m < r:
                right = mid
            else:
                right -= 1

        return result


def main():
    inputs = ([1, 3, 5], [2, 2, 2, 0, 1], [1])

    for nums in inputs:
        result = Solution().findMin(nums)
        print(result)


if __name__ == "__main__":
    main()
