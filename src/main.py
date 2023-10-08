from __future__ import annotations
from typing import Optional, List


class Solution:
    def maxDotProduct(self, nums1: List[int], nums2: List[int]) -> int:
        max0 = -2147483648
        max1 = -2147483648
        min0 = 2147483647
        min1 = 2147483647

        for num in nums1:
            max0 = max(max0, num)
            min0 = min(min0, num)

        for num in nums2:
            max1 = max(max1, num)
            min1 = min(min1, num)

        if (max0 < 0) and (min1 > 0):
            return max0 * min1

        if (min0 > 0) and (max1 < 0):
            return min0 * max1

        m = len(nums2) + 1
        prev_dp = [0] * m
        dp = prev_dp
        for i in range(len(nums1) - 1, -1, -1):
            dp = [0] * m
            for j in range(m - 2, -1, -1):
                use = nums1[i] * nums2[j] + prev_dp[j + 1]
                dp[j] = max(use, max(prev_dp[j], dp[j + 1]))

            prev_dp = dp

        return dp[0]


def main():
    inputs = (
        ([2, 1, -2, 5], [3, 0, -6]),
        ([3, -2], [2, -6, 7]),
        ([-1, -1], [1, 1]),
    )

    for nums1, nums2 in inputs:
        result = Solution().maxDotProduct(nums1, nums2)
        print(result)


if __name__ == "__main__":
    main()
