from __future__ import annotations
from typing import List


class Solution:
    def minPairSum(self, nums: List[int]) -> int:
        nums.sort()

        l = 0
        r = len(nums) - 1
        result = 0
        while l < r:
            result = max(result, nums[l] + nums[r])
            l += 1
            r -= 1

        return result


def main():
    inputs = ([3, 5, 2, 3], [3, 5, 4, 2, 4, 6])

    for nums in inputs:
        result = Solution().minPairSum(nums)
        print(result)


if __name__ == "__main__":
    main()
