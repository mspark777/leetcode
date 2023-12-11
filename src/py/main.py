from __future__ import annotations
from typing import List


class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        n = len(nums)
        left = 1
        right = 1
        result = nums[0]
        for i in range(n):
            if left == 0:
                left = 1
            if right == 0:
                right = 1

            left *= nums[i]
            right *= nums[n - 1 - i]

            result = max(result, left, right)

        return result


def main():
    inputs = ([2, 3, -2, 4], [-2, 0, -1])

    for nums in inputs:
        result = Solution().maxProduct(nums)
        print(result)


if __name__ == "__main__":
    main()
