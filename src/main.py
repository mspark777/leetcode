from __future__ import annotations
from typing import List


class Solution:
    def maxSubarraySumCircular(self, nums: List[int]) -> int:
        curmax = 0
        curmin = 0
        maxsum = nums[0]
        minsum = nums[0]
        sum = 0

        for num in nums:
            curmax = max(curmax, 0) + num
            maxsum = max(curmax, maxsum)
            curmin = min(curmin, 0) + num
            minsum = min(curmin, minsum)
            sum += num

        return maxsum if sum == minsum else max(maxsum, sum - minsum)


def main():
    inputs: list[list[int]] = [[1, -2, 3, -2], [5, -3, 5], [-3, -2, -3]]
    for nums in inputs:
        solution = Solution()
        result = solution.maxSubarraySumCircular(nums)
        print(result)


if __name__ == "__main__":
    main()
