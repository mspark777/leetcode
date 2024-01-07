from __future__ import annotations
from typing import List
from collections import defaultdict


class Solution:
    def numberOfArithmeticSlices(self, nums: List[int]) -> int:
        n = len(nums)
        result = 0
        dp = [defaultdict[int, int](int) for _ in range(n)]

        for i in range(1, n):
            for j in range(i):
                diff = nums[i] - nums[j]
                dp[i][diff] += 1
                if diff in dp[j]:
                    dp[i][diff] += dp[j][diff]
                    result += dp[j][diff]

        return result


def main():
    input = ([2, 4, 6, 8, 10], [7, 7, 7, 7, 7])

    for nums in input:
        result = Solution().numberOfArithmeticSlices(nums)
        print(result)


if __name__ == "__main__":
    main()
